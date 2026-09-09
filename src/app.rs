use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use eframe::egui::{self, Color32, RichText, Stroke};
use kemdara::{BenchmarkReport, ExperimentCategory, Maturity, registry, run_benchmarks};

const INK: Color32 = Color32::from_rgb(225, 232, 240);
const MUTED: Color32 = Color32::from_rgb(145, 158, 171);
const ACCENT: Color32 = Color32::from_rgb(64, 201, 176);
const PQ_ACCENT: Color32 = Color32::from_rgb(172, 126, 241);
const PANEL: Color32 = Color32::from_rgb(23, 29, 38);

pub struct KemdaraApp {
    iterations: String,
    status: String,
    report: Option<BenchmarkReport>,
    pending: Option<Receiver<BenchmarkReport>>,
}

impl KemdaraApp {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        configure_style(&context.egui_ctx);
        Self {
            iterations: "100".into(),
            status: "Ready — select an iteration count and run the local suite.".into(),
            report: None,
            pending: None,
        }
    }

    fn start_benchmark(&mut self) {
        let Ok(iterations) = self.iterations.trim().parse::<usize>() else {
            self.status = "Iteration count must be a whole number.".into();
            return;
        };
        if !(1..=10_000).contains(&iterations) {
            self.status = "Choose between 1 and 10,000 iterations.".into();
            return;
        }

        let (sender, receiver) = mpsc::channel();
        self.pending = Some(receiver);
        self.status = format!("Running up to {iterations} verified workloads per algorithm…");
        std::thread::spawn(move || {
            let _ = sender.send(run_benchmarks(iterations));
        });
    }

    fn poll_benchmark(&mut self, context: &egui::Context) {
        let Some(received) = self.pending.as_ref().map(Receiver::try_recv) else {
            return;
        };

        match received {
            Ok(report) => {
                let passed = report
                    .results
                    .iter()
                    .filter(|result| result.successful)
                    .count();
                self.status = format!(
                    "Complete — {passed}/{} workloads passed their correctness checks.",
                    report.results.len()
                );
                self.report = Some(report);
                self.pending = None;
            }
            Err(mpsc::TryRecvError::Empty) => {
                context.request_repaint_after(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                self.status = "The benchmark worker stopped unexpectedly.".into();
                self.pending = None;
            }
        }
    }
}

impl eframe::App for KemdaraApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_benchmark(ui.ctx());
        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, Color32::from_rgb(17, 22, 30));

        egui::Frame::new()
            .fill(Color32::from_rgb(13, 18, 25))
            .inner_margin(egui::Margin::symmetric(18, 14))
            .show(ui, |ui| {
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("KEMDARA").color(ACCENT).strong().size(24.0));
                    ui.label(RichText::new("crypto experiment workbench").color(MUTED));
                });
                ui.add_space(12.0);
            });

        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(18, 10))
            .show(ui, |ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Run verified cryptographic workloads on this machine")
                        .color(INK)
                        .size(20.0)
                        .strong(),
                );
                ui.label(
                    RichText::new(
                        "Results are grouped by workload; compare algorithms within a category, not across unlike operations.",
                    )
                    .color(MUTED),
                );
                ui.add_space(14.0);

                egui::Frame::new()
                    .fill(PANEL)
                    .stroke(Stroke::new(1.0, Color32::from_rgb(48, 59, 72)))
                    .inner_margin(14.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Iterations per algorithm");
                            ui.add_enabled(
                                self.pending.is_none(),
                                egui::TextEdit::singleline(&mut self.iterations)
                                    .desired_width(90.0),
                            );
                            if ui
                                .add_enabled(
                                    self.pending.is_none(),
                                    egui::Button::new("Run suite"),
                                )
                                .clicked()
                            {
                                self.start_benchmark();
                            }
                            if self.pending.is_some() {
                                ui.spinner();
                            }
                        });
                        ui.add_space(6.0);
                        ui.label(RichText::new(&self.status).color(MUTED));
                    });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(16.0);
                    if let Some(report) = &self.report {
                        report_view(ui, report);
                    } else {
                        algorithm_overview(ui);
                    }

                    ui.add_space(18.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "Research software only. Experimental constructions are isolated and must not protect production traffic.",
                        )
                        .color(Color32::from_rgb(232, 174, 92)),
                    );
                });
            });
    }
}

fn algorithm_overview(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("Included adapters")
            .color(INK)
            .strong()
            .size(17.0),
    );
    ui.add_space(8.0);
    egui::Grid::new("algorithm_overview")
        .num_columns(4)
        .spacing([28.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Algorithm");
            ui.strong("Standard");
            ui.strong("Category");
            ui.strong("Maturity");
            ui.end_row();
            for algorithm in registry() {
                let info = algorithm.info();
                ui.label(info.name);
                ui.label(info.standard);
                ui.label(info.category.label());
                ui.colored_label(
                    if info.maturity == Maturity::Experimental {
                        Color32::from_rgb(232, 174, 92)
                    } else {
                        MUTED
                    },
                    info.maturity.label(),
                );
                ui.end_row();
            }
        });
}

fn report_view(ui: &mut egui::Ui, report: &BenchmarkReport) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Local result").color(INK).strong().size(17.0));
        ui.label(
            RichText::new(format!(
                "{} · {} · {} logical cores",
                report.machine.os, report.machine.architecture, report.machine.logical_cores
            ))
            .color(MUTED),
        );
        if !report.machine.target_features.is_empty() {
            ui.label(
                RichText::new(report.machine.target_features.join(", "))
                    .color(Color32::from_rgb(105, 174, 211)),
            );
        }
    });
    ui.add_space(12.0);

    const CATEGORIES: [ExperimentCategory; 7] = [
        ExperimentCategory::KeyEstablishment,
        ExperimentCategory::PayloadEncryption,
        ExperimentCategory::Hash,
        ExperimentCategory::KeyDerivation,
        ExperimentCategory::PasswordDerivation,
        ExperimentCategory::DigitalSignature,
        ExperimentCategory::HybridKeyEstablishment,
    ];

    for category in CATEGORIES {
        let category_results: Vec<_> = report
            .results
            .iter()
            .filter(|result| result.category == category)
            .collect();
        if category_results.is_empty() {
            continue;
        }
        let max_mean = category_results
            .iter()
            .map(|result| result.mean_ns)
            .max()
            .unwrap_or(1)
            .max(1);
        ui.add_space(10.0);
        ui.label(RichText::new(category.label()).color(INK).strong());
        for result in category_results {
            let color = if result.maturity == Maturity::Experimental {
                Color32::from_rgb(232, 174, 92)
            } else if result.quantum_resistant {
                PQ_ACCENT
            } else {
                ACCENT
            };
            ui.horizontal(|ui| {
                ui.add_sized(
                    [175.0, 20.0],
                    egui::Label::new(RichText::new(result.algorithm).strong()),
                );
                let width = (ui.available_width() - 275.0).max(40.0);
                let fraction = result.mean_ns as f32 / max_mean as f32;
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(width, 14.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, 0.0, Color32::from_rgb(38, 47, 58));
                let filled = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(width * fraction, rect.height()),
                );
                ui.painter().rect_filled(filled, 0.0, color);
                ui.label(format_duration(result.mean_ns));
                ui.colored_label(
                    if result.successful {
                        ACCENT
                    } else {
                        Color32::from_rgb(235, 104, 104)
                    },
                    if result.successful {
                        "verified"
                    } else {
                        "failed"
                    },
                );
            });
            ui.label(
                RichText::new(format!(
                    "{} · {} samples · {}",
                    result.maturity.label(),
                    result.iterations,
                    result.workload
                ))
                .color(MUTED)
                .small(),
            );
            ui.add_space(5.0);
        }
    }

    ui.add_space(10.0);
    egui::Grid::new("result_table")
        .num_columns(6)
        .spacing([24.0, 8.0])
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Algorithm");
            ui.strong("Median");
            ui.strong("P95");
            ui.strong("Ops/sec");
            ui.strong("Samples");
            ui.strong("Standard");
            ui.end_row();
            for result in &report.results {
                ui.label(result.algorithm);
                ui.label(format_duration(result.median_ns));
                ui.label(format_duration(result.p95_ns));
                ui.label(format!("{:.1}", result.operations_per_second));
                ui.label(result.iterations.to_string());
                ui.label(result.standard);
                ui.end_row();
            }
        });
}

fn format_duration(nanoseconds: u128) -> String {
    if nanoseconds >= 1_000_000 {
        format!("{:.2} ms", nanoseconds as f64 / 1_000_000.0)
    } else if nanoseconds >= 1_000 {
        format!("{:.1} µs", nanoseconds as f64 / 1_000.0)
    } else {
        format!("{nanoseconds} ns")
    }
}

fn configure_style(context: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(17, 22, 30);
    visuals.window_fill = PANEL;
    visuals.selection.bg_fill = Color32::from_rgb(35, 116, 105);
    visuals.widgets.active.bg_fill = Color32::from_rgb(39, 145, 128);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(42, 91, 87);
    context.set_visuals(visuals);

    context.style_mut_of(egui::Theme::Dark, |style| {
        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 8.0);
    });
}
