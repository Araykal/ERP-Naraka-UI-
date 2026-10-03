#![allow(float_literal_f32_fallback)]
//该代码可以看出来 ERP 外挂客户端有多脑残，李阳 编写爱编写奇异搞笑 永劫无间 DMA 辅助。。。。
use eframe::egui::{
    self, Align, Align2, Color32, FontId, Frame, Layout, Margin, RichText, Rounding, Stroke, Ui,
    Vec2,
};
use eframe::egui::{FontData, FontDefinitions, FontFamily, ViewportCommand};
use std::time::Instant;

const BG: Color32 = Color32::from_rgb(11, 15, 23);
const SIDEBAR: Color32 = Color32::from_rgb(17, 23, 34);
const PANEL: Color32 = Color32::from_rgb(22, 29, 42);
const PANEL_SOFT: Color32 = Color32::from_rgb(27, 35, 50);
const BORDER: Color32 = Color32::from_rgb(45, 55, 74);
const TEXT: Color32 = Color32::from_rgb(236, 241, 247);
const MUTED: Color32 = Color32::from_rgb(143, 155, 175);
const MINT: Color32 = Color32::from_rgb(105, 228, 194);
const VIOLET: Color32 = Color32::from_rgb(161, 132, 255);
const AMBER: Color32 = Color32::from_rgb(255, 184, 92);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Perspective,
    Parry,
    Combos,
    Memory,
}

impl Section {
    const ALL: [Section; 4] = [Self::Perspective, Self::Parry, Self::Combos, Self::Memory];
    fn title(self) -> &'static str {
        match self {
            Self::Perspective => "透视类",
            Self::Parry => "振刀类",
            Self::Combos => "连招类",
            Self::Memory => "内存类",
        }
    }
    fn subtitle(self) -> &'static str {
        match self {
            Self::Perspective => "?",
            Self::Parry => "?",
            Self::Combos => "?",
            Self::Memory => "?",
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Self::Perspective => "◉",
            Self::Parry => "✦",
            Self::Combos => "⌁",
            Self::Memory => "▣",
        }
    }
    fn accent(self) -> Color32 {
        match self {
            Self::Perspective => MINT,
            Self::Parry => AMBER,
            Self::Combos => VIOLET,
            Self::Memory => Color32::from_rgb(107, 177, 255),
        }
    }
}

struct NarakaUi {
    active: Section,
    started_at: Instant,
    transition: f32,
    perspective_flags: [bool; 10],
    perspective_colors: [Color32; 2],
    resize_start: Option<(egui::Pos2, Vec2)>,
}

impl NarakaUi {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        if let Ok(bytes) = std::fs::read(r"C:\Windows\Fonts\msyh.ttc") {
            fonts
                .font_data
                .insert("cjk".to_owned(), FontData::from_owned(bytes));
            for family in [FontFamily::Proportional, FontFamily::Monospace] {
                if let Some(list) = fonts.families.get_mut(&family) {
                    list.insert(0, "cjk".to_owned());
                }
            }
            _cc.egui_ctx.set_fonts(fonts);
        }
        Self {
            active: Section::Perspective,
            started_at: Instant::now(),
            transition: 1.0,
            perspective_flags: [
                true, true, false, true, false, true, false, true, false, true,
            ],
            perspective_colors: [
                Color32::from_rgb(105, 228, 194),
                Color32::from_rgb(255, 184, 92),
            ],
            resize_start: None,
        }
    }

    fn card_frame() -> Frame {
        Frame::none()
            .fill(PANEL)
            .stroke(Stroke::new(1.0, BORDER))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(18.0))
    }

    fn pill(ui: &mut Ui, text: &str, color: Color32) {
        Frame::none()
            .fill(color.linear_multiply(0.14))
            .rounding(Rounding::same(7.0))
            .inner_margin(Margin {
                left: 9.0,
                right: 9.0,
                top: 5.0,
                bottom: 5.0,
            })
            .show(ui, |ui| {
                ui.label(
                    RichText::new(text)
                        .color(color)
                        .font(FontId::proportional(11.0))
                        .strong(),
                );
            });
    }

    fn draw_logo(ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(44.0), egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, Rounding::same(12.0), Color32::from_rgb(27, 34, 49));
        painter.rect_stroke(rect, Rounding::same(12.0), Stroke::new(1.0, BORDER));
        painter.rect_stroke(
            rect.shrink(9.0),
            Rounding::same(6.0),
            Stroke::new(2.0, MINT),
        );
        painter.line_segment(
            [
                egui::pos2(rect.left() + 15.0, rect.top() + 28.0),
                egui::pos2(rect.left() + 22.0, rect.top() + 15.0),
            ],
            Stroke::new(2.4, TEXT),
        );
        painter.line_segment(
            [
                egui::pos2(rect.left() + 22.0, rect.top() + 15.0),
                egui::pos2(rect.left() + 29.0, rect.top() + 28.0),
            ],
            Stroke::new(2.4, TEXT),
        );
        painter.circle_filled(rect.right_top() + egui::vec2(-5.0, 5.0), 3.0, AMBER);
    }

    fn render_sidebar(&mut self, ui: &mut Ui) {
        ui.set_min_width(222.0);
        ui.vertical(|ui| {
            ui.add_space(24.0);
            ui.horizontal(|ui| {
                Self::draw_logo(ui);
                ui.add_space(10.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("ERP").size(20.0).strong().color(TEXT));
                    ui.label(
                        RichText::new("NARAKA CONTROL")
                            .size(9.0)
                            .extra_letter_spacing(1.4)
                            .color(MUTED),
                    );
                });
            });
            ui.add_space(36.0);
            ui.label(RichText::new("工作台").size(11.0).color(MUTED).strong());
            ui.add_space(9.0);
            for section in Section::ALL {
                let selected = self.active == section;
                let accent = section.accent();
                let response = Frame::none()
                    .fill(if selected {
                        PANEL_SOFT
                    } else {
                        Color32::TRANSPARENT
                    })
                    .rounding(Rounding::same(11.0))
                    .inner_margin(Margin {
                        left: 12.0,
                        right: 10.0,
                        top: 11.0,
                        bottom: 11.0,
                    })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(section.icon()).size(19.0).color(if selected {
                                accent
                            } else {
                                MUTED
                            }));
                            ui.add_space(9.0);
                            ui.label(
                                RichText::new(section.title())
                                    .size(14.0)
                                    .color(if selected { TEXT } else { MUTED }),
                            );
                            if selected {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new("›").size(19.0).color(accent));
                                });
                            }
                        });
                    })
                    .response;
                if response.interact(egui::Sense::click()).clicked() {
                    self.active = section;
                    self.transition = 0.0;
                }
                ui.add_space(4.0);
            }
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.add_space(21.0);
                Frame::none()
                    .fill(Color32::from_rgb(24, 31, 44))
                    .rounding(Rounding::same(13.0))
                    .inner_margin(14.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.painter().circle_filled(
                                egui::pos2(ui.cursor().left() + 6.0, ui.cursor().top() + 8.0),
                                4.0,
                                MINT,
                            );
                            ui.add_space(17.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new("").size(12.0).color(TEXT));
                                ui.label(RichText::new("PREVIEW MODE").size(9.0).color(MUTED));
                            });
                        });
                    });
                ui.label(
                    RichText::new("v0.3.0  ·  SHIT UI")
                        .size(9.0)
                        .color(Color32::from_rgb(92, 103, 123)),
                );
            });
        });
    }

    fn render_header(&mut self, ui: &mut Ui) {
        let _ = ui;
    }

    fn render_stat_row(
        &self,
        ui: &mut Ui,
        title: &str,
        value: &str,
        detail: &str,
        accent: Color32,
    ) {
        ui.horizontal(|ui| {
            ui.allocate_ui(Vec2::new(7.0, 38.0), |ui| {
                ui.painter()
                    .rect_filled(ui.max_rect(), Rounding::same(4.0), accent);
            });
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label(RichText::new(title).size(12.0).color(MUTED));
                ui.label(RichText::new(value).size(19.0).strong().color(TEXT));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(detail).size(11.0).color(accent));
            });
        });
    }

    fn toggle_row(
        ui: &mut Ui,
        title: &str,
        description: &str,
        enabled: &mut bool,
        accent: Color32,
    ) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(title).size(13.0).color(TEXT));
                ui.label(RichText::new(description).size(11.0).color(MUTED));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let response = ui.checkbox(enabled, "");
                if *enabled {
                    ui.painter().circle_filled(
                        response.rect.right_center() - egui::vec2(7.0, 0.0),
                        3.0,
                        accent,
                    );
                }
            });
        });
    }

    fn glass_frame() -> Frame {
        Frame::none()
            .fill(Color32::from_rgba_unmultiplied(25, 34, 49, 188))
            .stroke(Stroke::new(
                1.0_f32,
                Color32::from_rgba_unmultiplied(104, 127, 157, 100),
            ))
            .rounding(Rounding::same(20.0))
            .inner_margin(Margin::same(22.0))
    }

    fn animated_toggle(ui: &mut Ui, label: &str, enabled: &mut bool, accent: Color32) {
        ui.horizontal(|ui| {
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(48.0, 26.0), egui::Sense::click());
            if response.clicked() {
                *enabled = !*enabled;
            }
            let t = ui.ctx().animate_bool(response.id, *enabled);
            let off = Color32::from_rgb(43, 52, 68);
            let track: Color32 = egui::lerp(
                egui::Rgba::from(off)..=egui::Rgba::from(accent.linear_multiply(0.72)),
                t,
            )
            .into();
            ui.painter().rect_filled(rect, Rounding::same(13.0), track);
            ui.painter().circle_filled(
                egui::pos2(rect.left() + 13.0 + t * 22.0, rect.center().y),
                9.0,
                Color32::from_rgb(240, 246, 250),
            );
            ui.add_space(12.0);
            ui.label(
                RichText::new(label)
                    .size(13.0)
                    .color(if *enabled { TEXT } else { MUTED }),
            );
        });
    }

    fn perspective_setting(
        ui: &mut Ui,
        label: &str,
        description: &str,
        enabled: &mut bool,
        accent: Color32,
        color: Option<&mut Color32>,
    ) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(label).size(14.0).color(TEXT));
                ui.label(RichText::new(description).size(10.5).color(MUTED));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(color) = color {
                    ui.color_edit_button_srgba(color);
                    ui.add_space(10.0);
                }
                Self::animated_toggle(ui, "", enabled, accent);
            });
        });
    }

    fn render_content(&mut self, ui: &mut Ui) {
        let accent = self.active.accent();
        match self.active {
            Section::Perspective => self.render_perspective(ui, accent),
            Section::Parry => self.render_parry(ui, accent),
            Section::Combos => self.render_combos(ui, accent),
            Section::Memory => self.render_memory(ui, accent),
        }
    }

    fn render_perspective(&mut self, ui: &mut Ui, accent: Color32) {
        ui.columns(2, |cols| {
            Self::glass_frame().show(&mut cols[0], |ui| {
                ui.label(RichText::new("玩家信息").size(16.0).strong().color(TEXT));
                ui.label(
                    RichText::new("select")
                        .size(11.0)
                        .color(MUTED),
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "玩家信息显示",
                    "昵称、状态与队伍信息",
                    &mut self.perspective_flags[0],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "英雄显示",
                    "显示英雄轮廓与名称",
                    &mut self.perspective_flags[1],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "骨骼显示",
                    "显示骨骼关键点与连线",
                    &mut self.perspective_flags[2],
                    accent,
                    Some(&mut self.perspective_colors[0]),
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "距离显示",
                    "显示目标到自身的距离",
                    &mut self.perspective_flags[3],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "背身预警",
                    "目标背向时显示提醒",
                    &mut self.perspective_flags[4],
                    accent,
                    None,
                );
            });
            Self::glass_frame().show(&mut cols[1], |ui| {
                ui.label(RichText::new("物资信息").size(16.0).strong().color(TEXT));
                ui.label(
                    RichText::new("自定义物资与装备的显示层")
                        .size(11.0)
                        .color(MUTED),
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "物资显示",
                    "显示附近可拾取物资",
                    &mut self.perspective_flags[5],
                    accent,
                    Some(&mut self.perspective_colors[1]),
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "武器显示",
                    "显示武器名称与品质",
                    &mut self.perspective_flags[6],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "魂玉显示",
                    "显示魂玉与稀有度",
                    &mut self.perspective_flags[7],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "物资堆显示",
                    "标记物资堆位置",
                    &mut self.perspective_flags[8],
                    accent,
                    None,
                );
                ui.add_space(18.0);
                Self::perspective_setting(
                    ui,
                    "高价值提醒",
                    "突出稀有物资颜色",
                    &mut self.perspective_flags[9],
                    accent,
                    None,
                );
            });
        });
    }

    fn render_parry(&mut self, ui: &mut Ui, accent: Color32) {
    }

    fn render_combos(&mut self, ui: &mut Ui, accent: Color32) {

    }

    fn render_memory(&mut self, ui: &mut Ui, accent: Color32) {
        ui.columns(2, |cols| {
            Self::card_frame().show(&mut cols[0], |ui| {
                ui.label(RichText::new("配置档案").size(15.0).strong().color(TEXT));
                ui.label(
                    RichText::new("快速切换")
                        .size(11.0)
                        .color(MUTED),
                );
                ui.add_space(16.0);
                for (name, note, active) in [
                    ("", "", true),
                ] {
                    ui.horizontal(|ui| {
                        ui.painter().circle_filled(
                            ui.cursor().left_top() + egui::vec2(6.0, 10.0),
                            5.0,
                            if active { accent } else { BORDER },
                        );
                        ui.add_space(19.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(name).size(13.0).color(TEXT));
                            ui.label(RichText::new(note).size(11.0).color(MUTED));
                        });
                    });
                    ui.add_space(15.0);
                }
            });
        });
    }
}

impl eframe::App for NarakaUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
        let fade = (self.started_at.elapsed().as_secs_f32() * 3.0).min(1.0);
        self.transition = (self.transition + 0.12).min(1.0);
        let transition = self.transition * self.transition * (3.0 - 2.0 * self.transition);
        egui::SidePanel::left("navigation")
            .exact_width(242.0)
            .resizable(false)
            .frame(Frame::none().fill(SIDEBAR).inner_margin(Margin {
                left: 20.0,
                right: 18.0,
                top: 0.0,
                bottom: 0.0,
            }))
            .show(ctx, |ui| self.render_sidebar(ui));
        egui::CentralPanel::default()
            .frame(Frame::none().fill(BG).inner_margin(Margin {
                left: 30.0,
                right: 30.0,
                top: 29.0,
                bottom: 25.0,
            }))
            .show(ctx, |ui| {
                let (bar_rect, bar_response) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 38.0),
                    egui::Sense::drag(),
                );
                ui.painter().rect_filled(bar_rect, Rounding::ZERO, SIDEBAR);
                ui.painter().text(
                    bar_rect.left_center() + egui::vec2(2.0, 0.0),
                    Align2::LEFT_CENTER,
                    "ERP  /  NARAKA CONTROL",
                    FontId::proportional(11.0),
                    MUTED,
                );
                if bar_response.drag_started() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 74.0).max(0.0));
                    if ui.small_button("—").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                    }
                    if ui.small_button("×").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.add_space(11.0);
                self.render_header(ui);
                ui.add_space(25.0);
                ui.scope(|ui| {
                    ui.set_opacity(fade * transition);
                    ui.add_space((1.0 - transition) * 10.0);
                    self.render_content(ui);
                });
                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.label(
                        RichText::new("NARAKA  /  ERP CONTROL SURFACE")
                            .size(9.0)
                            .color(Color32::from_rgb(79, 91, 111)),
                    );
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ERP · Naraka Control")
            .with_inner_size([980.0, 780.0])
            .with_min_inner_size([900.0, 680.0])
            .with_decorations(false)
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "ERP · Naraka Control",
        options,
        Box::new(|cc| Ok(Box::new(NarakaUi::new(cc)))),
    )
}
