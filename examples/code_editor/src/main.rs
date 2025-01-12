use eframe::egui::{ Color32, FontId, TextFormat};
use eframe::{egui, NativeOptions};
use egui::{Align, Stroke};
use egui::epaint::text::cursor::Cursor;
use syntect::{
    highlighting::{Highlighter, ThemeSet},
    parsing::{ParseState, SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};
use syntect::easy::HighlightLines;
use syntect::highlighting::FontStyle;
use egui::ahash::HashMap;
use egui::text::{LayoutJob, LayoutSection};

struct CodeEditorResponse {
    range:TextRange,
}

impl CodeEditorResponse {
    pub fn new(range:TextRange) -> Self {
        Self { range }
    }
}
struct TextRange {
    start: Option<Cursor>,
    end: Cursor,
}
#[derive(Debug)]
struct SpanStyle {
    color: Color32,
    background_color: Option<Color32>,
    wave_underline: bool,
    italics: bool,
}
#[derive(Debug)]
struct Span {
    style: SpanStyle,
    range: std::ops::Range<usize>
}

pub enum ErrorType {
    ERROR,
    WARNING,
    INFO,
}

pub struct CodeError {
    message: String,
    error_type: ErrorType,
}

struct CodeEditor {
    buffer: String,
    syntax_highlighter: SyntaxHighlighter,
    font_size: f32,
    error_list: Vec<CodeError>,
    selected_range: TextRange,
}

struct SyntaxHighlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
    language: String,
}

impl SyntaxHighlighter {
    fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        Self {
            ps: syntax_set,
            ts: theme_set,
            language: "Rs".to_string(),
        }
    }

    pub fn highlight_text(&mut self, text: &str) -> Option<Vec<Span>> {
        let mut spans = Vec::new();
        let lan= self.language.as_str();
        let syntax = self
            .ps
            .find_syntax_by_name(lan)
            .or_else(|| self.ps.find_syntax_by_extension(lan))?;
        let theme = &self.ts.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);
        for line in LinesWithEndings::from(text) {
            for (style, range) in h.highlight_line(line, &self.ps).ok()? {
                let fg = style.foreground;
                let bg = style.background;
                let text_color = egui::Color32::from_rgb(fg.r, fg.g, fg.b);
                let italics = style.font_style.contains(FontStyle::ITALIC);
                spans.push(Span {
                    range: as_byte_range(text,range),
                    style: SpanStyle {
                        color: text_color,
                        background_color: None,
                        wave_underline: false,
                        italics,
                    },
                });
            }
        }
        Some(spans)
    }
}

impl CodeEditor {
    pub fn new() -> Self {
        let mut editor = Self {
            buffer: String::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
            font_size: 12.0,
            error_list: vec![],
            selected_range: TextRange {
                start: None,
                end: Default::default(),
            },
        };
        editor
    }

    pub fn load(&mut self,string: impl Into<String>) {
        self.buffer = string.into();
    }

    pub fn create_layout_job(&mut self) -> LayoutJob {
        let Self {
            buffer,
            syntax_highlighter,
            font_size,
            error_list,
            selected_range
        } = self;
        let len = buffer.len();
        let mut job = LayoutJob::default();
        let spans = self.syntax_highlighter.highlight_text(buffer.as_str());
        job.text = buffer.clone();

        let mut last_end = 0;
        for span in spans.unwrap_or(Vec::new()) {
            // Add the highlighted span
            let format = TextFormat {
                font_id: FontId::monospace(*font_size),
                color: span.style.color,
                background: span.style.background_color.unwrap_or(Color32::TRANSPARENT),
                wave_underline: if span.style.wave_underline {
                    Stroke::new(1.,Color32::RED)
                } else {
                    Stroke::NONE
                },
                italics: span.style.italics,
                underline: if span.style.italics {
                    Stroke::new(1.,span.style.color)
                } else {
                    Stroke::NONE
                },
                line_height:Some(20.),
                valign:Align::Center,
                ..Default::default()
            };

            job.sections.push(LayoutSection {
                leading_space: 0.0,
                byte_range: span.range,
                format,
            });
        }
        job
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> eframe::Result {
    let options = NativeOptions::default();
    eframe::run_native(
        "Egui LayoutJob Demo",
        options,
        Box::new(|_cc| Ok(Box::<LayoutJobApp>::new(LayoutJobApp::default()))),
    )
}

struct LayoutJobApp {
    pub editor: CodeEditor
}

impl Default for LayoutJobApp {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutJobApp {
    pub fn new() -> Self {
        let mut editor = CodeEditor::new();
        let str = r#"
fn render_gutter(&self, ui: &mut egui::Ui) {
    let line_count = self.buffer.content.to_string().lines().count();
    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
        for line_number in 1..=line_count {
        }
    });
}
        "#;
        editor.load(str);
        println!("{}",editor.buffer);
        Self {
            editor,
        }
    }
}

impl eframe::App for LayoutJobApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is a LayoutJob example:");
            ui.separator();
            //demo1 just render
            let text = self.editor.create_layout_job();
            ui.label(text);
        });
    }
}

fn as_byte_range(whole: &str, range: &str) -> std::ops::Range<usize> {
    let whole_start = whole.as_ptr() as usize;
    let range_start = range.as_ptr() as usize;
    assert!(whole_start <= range_start);
    assert!(range_start + range.len() <= whole_start + whole.len());
    let offset = range_start - whole_start;
    offset..(offset + range.len())
}