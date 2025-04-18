//! Skeleton component
//!
//! The Skeleton component is a temporary animation placeholder
//! for when a service call takes time to return data and we don't want
//! to block rendering the rest of the UI. It is intended to be used
//! inside an ExtFrame to fill the entire frame.
//!
//! Reference: [Fluent UI Skeleton](https://react.fluentui.dev/?path=/docs/components-skeleton--docs)

use std::sync::Arc;

use crate::egui::{self, Color32, Pos2, Response, Sense, Ui, Widget};
use crate::epaint::{self, Mesh, Rect, Stroke, StrokeKind, TextureId, Vertex};

/// 不同的 Skeleton 外观类型
#[derive(Clone, Debug)]
pub enum SkeletonShapeType {
    /// 矩形占位（默认）
    Rectangle,
    /// 正方形占位
    Square,
    /// 圆形占位
    Circle,
}

/// Skeleton 占位组件
///
/// - `base_color` 是背景色；
/// - `highlight_color` 是移动高光带的颜色；
/// - `animation_duration` 控制一个动画循环的时长（秒）；
/// - `shape_type` 指定占位的形状类型。
#[derive(Clone, Debug)]
pub struct Skeleton {
    pub base_color: Color32,
    pub highlight_color: Color32,
    pub animation_duration: f32,
    pub shape_type: SkeletonShapeType,
}

impl Default for Skeleton {
    fn default() -> Self {
        Self {
            base_color: Color32::from_gray(200),
            highlight_color: Color32::from_gray(230),
            animation_duration: 1.5, // seconds per cycle,
            shape_type: SkeletonShapeType::Rectangle,
        }
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_rect = ui.available_rect_before_wrap();
        let painter = ui.painter();

        match self.shape_type {
            SkeletonShapeType::Rectangle => {
                // 使用矩形+渐变高光效果
                let time = ui.input().time;
                let shimmer_phase = (time / self.animation_duration) % 1.0;
                let shimmer_width = 0.2 * available_rect.width();
                let shimmer_x = available_rect.left()
                    + shimmer_phase * (available_rect.width() + shimmer_width)
                    - shimmer_width;
                let x0 = available_rect.left();
                let x1 = shimmer_x.clamp(available_rect.left(), available_rect.right());
                let x2 = (shimmer_x + shimmer_width).clamp(available_rect.left(), available_rect.right());
                let x3 = available_rect.right();
                let top = available_rect.top();
                let bottom = available_rect.bottom();
                let mut mesh = Mesh::default();
                let uv = Pos2::new(0.0, 0.0);
                mesh.vertices.push(Vertex { pos: Pos2::new(x0, top), uv: uv, color: self.base_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x1, top), uv: uv, color: self.base_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x2, top), uv: uv, color: self.highlight_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x3, top), uv: uv, color: self.base_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x0, bottom), uv: uv, color: self.base_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x1, bottom), uv: uv, color: self.base_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x2, bottom), uv: uv, color: self.highlight_color });
                mesh.vertices.push(Vertex { pos: Pos2::new(x3, bottom), uv: uv, color: self.base_color });
                mesh.indices.extend_from_slice(&[0, 1, 5, 0, 5, 4]);
                mesh.indices.extend_from_slice(&[1, 2, 6, 1, 6, 5]);
                mesh.indices.extend_from_slice(&[2, 3, 7, 2, 7, 6]);
                painter.add(epaint::Shape::Mesh(Arc::new(mesh)));
                ui.allocate_rect(available_rect, Sense::hover())
            }
            SkeletonShapeType::Square => {
                // 在区域中绘制一个正方形占位
                let side = available_rect.width().min(available_rect.height());
                let square_rect = Rect::from_center_size(available_rect.center(), crate::egui::vec2(side, side));
                let shape = epaint::RectShape::new(
                    square_rect,
                    2.0,
                    self.base_color,
                    Stroke::NONE,
                    StrokeKind::Outside,
                );
                painter.add(shape.into());
                ui.allocate_rect(available_rect, Sense::hover())
            }
            SkeletonShapeType::Circle => {
                // 在区域中绘制一个圆形占位
                let radius = available_rect.width().min(available_rect.height()) / 2.0;
                let circle = epaint::Shape::Circle(epaint::CircleShape {
                    center: available_rect.center(),
                    radius,
                    fill: self.base_color,
                    stroke: Stroke::default(),
                });
                painter.add(circle);
                ui.allocate_rect(available_rect, Sense::hover())
            }
        }
    }
}

/// Trait for components that can display a skeleton placeholder while waiting for data.
pub trait HasSkeleton {
    /// Fills the provided area with default skeleton placeholders.
    ///
    /// 默认实现：在给定矩形内绘制带 gap 的多行矩形 skeleton，占位效果类似文本行占位。
    fn fill_ui(&self, ui: &mut Ui, rect: Rect) {
        let line_height = 16.0;
        let gap = 4.0;
        let mut y = rect.top();
        let mut shapes = Vec::new();
        while y + line_height <= rect.bottom() {
            let line_rect = Rect::from_min_size(Pos2::new(rect.left(), y), crate::egui::vec2(rect.width(), line_height));
            let skeleton_line = epaint::RectShape::new(
                line_rect,
                2.0,
                Color32::from_gray(220),
                Stroke::NONE,
                StrokeKind::Outside,
            );
            shapes.push(skeleton_line.into());
            y += line_height + gap;
        }
        ui.painter().add(epaint::Shape::Vec(shapes));
        ui.allocate_rect(rect, Sense::hover());
    }
}

/// 为 ExtFrame 实现 HasSkeleton trait（默认实现）
use crate::egui::containers::frame_ext::ExtFrame;
impl HasSkeleton for ExtFrame {}
