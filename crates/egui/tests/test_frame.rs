//! A test/demo for ExtFrame border/shadow variations and using Skeleton to fill ExtFrame.
//!
//! This example launches an eframe application which displays three frames:
//! 1. A frame with a solid border and an outer shadow.
//! 2. A frame with an inner shadow.
//! 3. A frame filled with a skeleton placeholder using the default HasSkeleton trait.

use eframe::egui;
use eframe::egui::{Color32, vec2};
use eframe::epi;

use crate::egui::containers::frame_ext::{ExtFrame, ExtStroke, ExtShadow, FrameSize, StrokeStyle, ShadowType};
use crate::egui::widgets::skeleton::{HasSkeleton};

struct TestFrameApp;

impl epi::App for TestFrameApp {
    fn name(&self) -> &str {
        "Test ExtFrame and Skeleton Demo"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test ExtFrame with Various Borders and Skeleton Fill");
            ui.separator();

            // Example 1: A frame with a solid border and an outer shadow.
            let frame1 = ExtFrame {
                inner_margin: egui::Margin::symmetric(8.0, 8.0),
                fill: Color32::from_rgb(240, 240, 240),
                stroke: ExtStroke {
                    width: 2.0,
                    color: Color32::BLACK,
                    style: StrokeStyle::Solid,
                },
                rounding: egui::Rounding::same(4.0),
                outer_margin: egui::Margin::same(4.0),
                shadows: vec![
                    ExtShadow {
                        offset: vec2(4.0, 4.0),
                        blur_radius: 3.0,
                        spread: 2.0,
                        color: Color32::DARK_GRAY,
                        shadow_type: ShadowType::Outer,
                    }
                ],
                embedded: None,
                size_mode: FrameSize::Fixed { width: 300.0, height: 150.0 },
            };
            ui.label("Frame Example 1: Solid Border with Outer Shadow");
            frame1.end(ui);
            ui.add_space(20.0);

            // Example 2: A frame with an inner shadow.
            let frame2 = ExtFrame {
                inner_margin: egui::Margin::symmetric(8.0, 8.0),
                fill: Color32::WHITE,
                stroke: ExtStroke {
                    width: 3.0,
                    color: Color32::from_rgb(100, 100, 100),
                    style: StrokeStyle::Solid,
                },
                rounding: egui::Rounding::same(8.0),
                outer_margin: egui::Margin::same(4.0),
                shadows: vec![
                    ExtShadow {
                        offset: vec2(0.0, 0.0),
                        blur_radius: 6.0,
                        spread: 3.0,
                        color: Color32::LIGHT_GRAY,
                        shadow_type: ShadowType::Inner,
                    }
                ],
                embedded: None,
                size_mode: FrameSize::Fixed { width: 300.0, height: 150.0 },
            };
            ui.label("Frame Example 2: Border with Inner Shadow");
            frame2.end(ui);
            ui.add_space(20.0);

            // Example 3: A frame whose interior is filled using the skeleton placeholder.
            let frame3 = ExtFrame {
                inner_margin: egui::Margin::symmetric(10.0, 10.0),
                fill: Color32::from_rgb(250, 250, 250),
                stroke: ExtStroke {
                    width: 2.0,
                    color: Color32::from_rgb(180, 180, 180),
                    style: StrokeStyle::Solid,
                },
                rounding: egui::Rounding::same(6.0),
                outer_margin: egui::Margin::same(6.0),
                shadows: vec![],
                embedded: None,
                size_mode: FrameSize::Fixed { width: 300.0, height: 150.0 },
            };
            ui.label("Frame Example 3: Frame with Skeleton Fill");
            // Allocate space for frame3.
            let rect = ui.allocate_space(vec2(300.0, 150.0));
            // Paint the frame border.
            frame3.paint(ui);
            // Fill the interior with the default skeleton fill.
            <ExtFrame as HasSkeleton>::fill_ui(&frame3, ui, rect);
        });
    }
}

fn main() {
    let app = TestFrameApp;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
