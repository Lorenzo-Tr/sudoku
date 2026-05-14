use iced::alignment::{Horizontal, Vertical};
use iced::border;
use iced::keyboard::{self, key};
use iced::widget::{button, column, container, row, svg, text};
use iced::{
    Background, Color, Element, Font, Length, Subscription, Task, Theme, application, font,
};
use std::time::{Duration, Instant};

use crate::board::{Board, Cell};
use crate::puzzle::{Puzzle, easy_puzzle, hard_puzzle, medium_puzzle, starter_puzzle};

const CELL_SIZE: f32 = 64.0;
const THIN_LINE: f32 = 1.0;
const THICK_LINE: f32 = 3.0;
const SIDE_PANEL_WIDTH: f32 = 290.0;
const TOOL_BUTTON_SIZE: f32 = 46.0;
const DIGIT_BUTTON_SIZE: f32 = 90.0;
const PRIMARY_BUTTON_HEIGHT: f32 = 46.0;
const BOARD_BG: Color = Color::from_rgb(0.93, 0.95, 0.98);
const CELL_BG: Color = Color::from_rgb(0.98, 0.99, 1.00);
const FIXED_BG: Color = Color::from_rgb(0.98, 0.99, 1.00);
const INVALID_BG: Color = Color::from_rgb(0.97, 0.78, 0.78);
const PRIMARY_BG: Color = Color::from_rgb(0.56, 0.72, 0.94);
const HIGHLIGHT_BG: Color = Color::from_rgb(0.84, 0.89, 0.95);
const BOX_HIGHLIGHT_BG: Color = Color::from_rgb(0.88, 0.92, 0.97);
const SAME_VALUE_BG: Color = Color::from_rgb(0.74, 0.82, 0.92);
const LINE_COLOR: Color = Color::from_rgb(0.27, 0.33, 0.45);
const TEXT_COLOR: Color = Color::from_rgb(0.24, 0.31, 0.43);
const MUTED_TEXT_COLOR: Color = Color::from_rgb(0.42, 0.48, 0.58);
const USER_TEXT_COLOR: Color = Color::from_rgb(0.20, 0.42, 0.73);
const NOTE_TEXT_COLOR: Color = Color::from_rgb(0.52, 0.58, 0.70);
const ACTIVE_NOTE_TEXT_COLOR: Color = Color::from_rgb(0.16, 0.38, 0.70);
const ERROR_TEXT_COLOR: Color = Color::from_rgb(0.73, 0.18, 0.18);
const APP_BG: Color = Color::from_rgb(0.97, 0.97, 0.98);
const PANEL_BG: Color = Color::from_rgb(0.90, 0.93, 0.98);
const TOOL_BG: Color = Color::from_rgb(0.91, 0.94, 0.98);
const TOOL_ACTIVE_BG: Color = Color::from_rgb(0.39, 0.51, 0.76);
const DIGIT_BG: Color = Color::from_rgb(0.91, 0.94, 0.98);
const DIGIT_ACTIVE_BG: Color = Color::from_rgb(0.63, 0.78, 0.95);

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Value,
    Notes,
}

impl InputMode {
    fn toggle(self) -> Self {
        match self {
            Self::Value => Self::Notes,
            Self::Notes => Self::Value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationMode {
    Live,
    Manual,
}

impl ValidationMode {
    fn toggle(self) -> Self {
        match self {
            Self::Live => Self::Manual,
            Self::Manual => Self::Live,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameStatus {
    Playing,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        }
    }

    fn puzzle(self) -> [[Option<u8>; 9]; 9] {
        match self {
            Self::Easy => easy_puzzle(),
            Self::Medium => medium_puzzle(),
            Self::Hard => hard_puzzle(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    CellPressed(usize, usize, keyboard::Modifiers),
    StartGamePressed,
    ResetPressed,
    NewGamePressed,
    RetryPressed,
    ClearPressed,
    ToggleModePressed,
    ToggleValidationModePressed,
    SubmitPressed,
    DigitPressed(u8),
    SetDifficulty(Difficulty),
    EventOccurred(iced::Event),
}

pub struct SudokuApp {
    board: Board,
    current_puzzle: Puzzle,
    selected: Vec<Position>,
    modifiers: keyboard::Modifiers,
    mode: InputMode,
    validation_mode: ValidationMode,
    difficulty: Difficulty,
    show_validation_result: bool,
    started: bool,
    status: GameStatus,
    started_at: Option<Instant>,
    finished_elapsed: Option<Duration>,
}

impl Default for SudokuApp {
    fn default() -> Self {
        let puzzle = starter_puzzle();

        Self {
            board: Board::from_start(puzzle),
            current_puzzle: puzzle,
            selected: vec![(0, 0)],
            modifiers: keyboard::Modifiers::default(),
            mode: InputMode::Value,
            validation_mode: ValidationMode::Manual,
            difficulty: Difficulty::Easy,
            show_validation_result: false,
            started: false,
            status: GameStatus::Playing,
            started_at: None,
            finished_elapsed: None,
        }
    }
}

pub fn run() -> iced::Result {
    application("Sudoku", update, view)
        .theme(|_| Theme::TokyoNight)
        .subscription(|_| subscription())
        .run()
}

fn subscription() -> Subscription<Message> {
    iced::event::listen().map(Message::EventOccurred)
}

fn update(app: &mut SudokuApp, message: Message) -> Task<Message> {
    match message {
        Message::CellPressed(row, col, modifiers) => {
            if app.status == GameStatus::Finished {
                return Task::none();
            }

            let position = (row, col);
            let command_pressed = modifiers.command();
            let shift_pressed = modifiers.shift();

            if app.mode == InputMode::Notes && shift_pressed {
                let start = primary_selection(&app.selected);
                select_range(&mut app.selected, start, position, &app.board);
            } else if app.mode == InputMode::Notes
                && command_pressed
                && !app.board.cell(row, col).fixed
            {
                if let Some(index) = app
                    .selected
                    .iter()
                    .position(|selected| *selected == position)
                {
                    if app.selected.len() > 1 {
                        app.selected.remove(index);
                    }
                } else {
                    app.selected.push(position);
                }
            } else {
                app.selected.clear();
                app.selected.push(position);
            }
        }
        Message::ResetPressed => {
            if app.status == GameStatus::Playing {
                app.board.clear_non_fixed();
                app.show_validation_result = false;
            }
        }
        Message::StartGamePressed => {
            start_game(app, app.difficulty.puzzle());
            app.started = true;
        }
        Message::NewGamePressed => {
            reset_game(app, app.difficulty.puzzle());
            app.started = false;
        }
        Message::RetryPressed => {
            start_game(app, app.current_puzzle);
            app.started = true;
        }
        Message::ClearPressed => clear_selection(app),
        Message::ToggleModePressed => toggle_input_mode(app),
        Message::ToggleValidationModePressed => {
            app.validation_mode = app.validation_mode.toggle();
            app.show_validation_result = false;
        }
        Message::SubmitPressed => {
            if can_submit(app) {
                finish_game(app);
            }
        }
        Message::DigitPressed(value) => {
            apply_digit(app, value);
        }
        Message::SetDifficulty(difficulty) => {
            if !app.started {
                app.difficulty = difficulty;
            }
        }
        Message::EventOccurred(event) => {
            if let iced::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) =
                event.clone()
            {
                app.modifiers = modifiers;
            }

            if app.started
                && let Some(action) = keyboard_action(event)
            {
                handle_action(app, action);
            }
        }
    }

    Task::none()
}

fn view(app: &SudokuApp) -> Element<'_, Message> {
    if !app.started {
        return view_setup_screen(app);
    }

    view_game_screen(app)
}

fn view_setup_screen(app: &SudokuApp) -> Element<'_, Message> {
    let difficulty_row = row![
        difficulty_button(app, Difficulty::Easy),
        difficulty_button(app, Difficulty::Medium),
        difficulty_button(app, Difficulty::Hard),
    ]
    .spacing(10);

    let validation_toggle = dual_toggle_button(
        app.validation_mode == ValidationMode::Live,
        "A",
        "Auto",
        "✓",
        "Submit",
        Message::ToggleValidationModePressed,
    );

    let panel = container(
        column![
            text("Sudoku")
                .size(42)
                .color(TEXT_COLOR)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            text("Choose your difficulty and submit mode")
                .size(15)
                .color(MUTED_TEXT_COLOR)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            column![
                text("Difficulty").size(13).color(MUTED_TEXT_COLOR),
                difficulty_row,
            ]
            .spacing(8),
            column![
                text("Submit mode").size(13).color(MUTED_TEXT_COLOR),
                validation_toggle,
            ]
            .spacing(8),
            primary_button("Start game", Message::StartGamePressed),
        ]
        .spacing(18)
        .padding(28)
        .width(Length::Fixed(420.0)),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: border::rounded(18).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    });

    container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(APP_BG)),
            ..Default::default()
        })
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn view_game_screen(app: &SudokuApp) -> Element<'_, Message> {
    let board = build_board(app);
    let side_panel = build_side_panel(app);

    let content = row![board, side_panel]
        .spacing(14)
        .padding(14)
        .align_y(Vertical::Top);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(APP_BG)),
            ..Default::default()
        })
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn build_side_panel(app: &SudokuApp) -> Element<'_, Message> {
    if app.status == GameStatus::Finished {
        return build_result_side_panel(app);
    }

    let can_submit = can_submit(app);
    let can_clear = can_clear_selection(app);
    let tool_row = row![
        tool_button(Icon::Reset, Message::ResetPressed),
        tool_button_enabled(Icon::Delete, Message::ClearPressed, can_clear),
        mode_tool_button(app.mode),
    ]
    .spacing(8);

    let keypad = column![build_keypad(), container("").height(Length::Fixed(14.0))].spacing(0);
    let controls = if app.validation_mode == ValidationMode::Manual {
        column![
            primary_button_enabled("Complete", Message::SubmitPressed, can_submit),
            tool_row,
            keypad,
            container("").height(Length::Fill),
            primary_button("New game", Message::NewGamePressed)
        ]
    } else {
        column![
            tool_row,
            keypad,
            container("").height(Length::Fill),
            primary_button("New game", Message::NewGamePressed)
        ]
    };

    container(controls.spacing(18).height(Length::Fixed(board_height())))
        .width(Length::Fixed(SIDE_PANEL_WIDTH))
        .into()
}

fn difficulty_button(app: &SudokuApp, difficulty: Difficulty) -> Element<'static, Message> {
    let active = app.difficulty == difficulty;
    button(
        container(
            text(difficulty.label())
                .size(12)
                .color(if active {
                    Color::WHITE
                } else {
                    USER_TEXT_COLOR
                })
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .padding([6, 4])
    .width(Length::FillPortion(1))
    .height(Length::Fixed(56.0))
    .style(move |_, _| button_style(active, TOOL_BG, TOOL_ACTIVE_BG))
    .on_press(Message::SetDifficulty(difficulty))
    .into()
}

fn build_result_side_panel(app: &SudokuApp) -> Element<'_, Message> {
    let won = app.board.is_complete();
    let title = if won { "Win" } else { "Defeat" };
    let title_color = if won {
        USER_TEXT_COLOR
    } else {
        ERROR_TEXT_COLOR
    };

    let recap = container(
        column![
            text(title)
                .size(32)
                .color(title_color)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            stat_row(
                "Time",
                format_duration(app.finished_elapsed.unwrap_or_default())
            ),
            stat_row("Errors", error_count(&app.board).to_string()),
            stat_row("Filled", format!("{}/81", filled_count(&app.board))),
        ]
        .spacing(14)
        .padding(18),
    )
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(PANEL_BG)),
        border: border::rounded(10).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    });

    container(
        column![
            recap,
            container("").height(Length::Fill),
            primary_button("Retry", Message::RetryPressed),
            primary_button("New game", Message::NewGamePressed)
        ]
        .spacing(10)
        .height(Length::Fixed(board_height())),
    )
    .width(Length::Fixed(SIDE_PANEL_WIDTH))
    .into()
}

fn stat_row<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![
        text(label).size(14).color(MUTED_TEXT_COLOR),
        text(value)
            .size(16)
            .color(TEXT_COLOR)
            .width(Length::Fill)
            .align_x(Horizontal::Right),
    ]
    .align_y(Vertical::Center)
    .into()
}

fn dual_toggle_button<'a>(
    left_active: bool,
    left_symbol: &'a str,
    left_label: &'a str,
    right_symbol: &'a str,
    right_label: &'a str,
    message: Message,
) -> Element<'a, Message> {
    let left = container(
        column![
            text(left_symbol).size(17).color(if left_active {
                Color::WHITE
            } else {
                USER_TEXT_COLOR
            }),
            text(left_label).size(10).color(if left_active {
                Color::WHITE
            } else {
                USER_TEXT_COLOR
            }),
        ]
        .spacing(2)
        .align_x(Horizontal::Center),
    )
    .width(Length::FillPortion(1))
    .padding([8, 6])
    .style(move |_| container::Style {
        background: Some(Background::Color(if left_active {
            TOOL_ACTIVE_BG
        } else {
            PANEL_BG
        })),
        border: border::rounded(10).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    });

    let right_active = !left_active;
    let right = container(
        column![
            text(right_symbol).size(17).color(if right_active {
                Color::WHITE
            } else {
                USER_TEXT_COLOR
            }),
            text(right_label).size(10).color(if right_active {
                Color::WHITE
            } else {
                USER_TEXT_COLOR
            }),
        ]
        .spacing(2)
        .align_x(Horizontal::Center),
    )
    .width(Length::FillPortion(1))
    .padding([8, 6])
    .style(move |_| container::Style {
        background: Some(Background::Color(if right_active {
            TOOL_ACTIVE_BG
        } else {
            PANEL_BG
        })),
        border: border::rounded(10).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    });

    button(row![left, right].spacing(4))
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fixed(54.0))
        .style(|_, _| button::Style {
            background: None,
            border: border::rounded(0).width(0).color(Color::TRANSPARENT),
            ..Default::default()
        })
        .on_press(message)
        .into()
}

#[derive(Debug, Clone, Copy)]
enum Icon {
    Reset,
    Delete,
    Notes,
}

fn tool_button<'a>(icon: Icon, message: Message) -> Element<'a, Message> {
    tool_button_enabled(icon, message, true)
}

fn tool_button_enabled<'a>(
    icon_kind: Icon,
    message: Message,
    enabled: bool,
) -> Element<'a, Message> {
    let icon_color = if enabled {
        USER_TEXT_COLOR
    } else {
        NOTE_TEXT_COLOR
    };

    let button = button(
        container(
            icon(icon_kind, icon_color)
                .width(Length::Fixed(22.0))
                .height(Length::Fixed(22.0)),
        )
        .width(Length::Fixed(TOOL_BUTTON_SIZE))
        .height(Length::Fixed(TOOL_BUTTON_SIZE))
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .padding(0)
    .width(Length::FillPortion(1))
    .height(Length::Fixed(TOOL_BUTTON_SIZE))
    .style(|_, status| disabled_aware_button_style(status, false, TOOL_BG, TOOL_ACTIVE_BG));

    if enabled {
        button.on_press(message).into()
    } else {
        button.into()
    }
}

fn mode_tool_button(mode: InputMode) -> Element<'static, Message> {
    let active = mode == InputMode::Notes;
    let icon_color = if active {
        Color::WHITE
    } else {
        USER_TEXT_COLOR
    };

    button(
        container(
            icon(Icon::Notes, icon_color)
                .width(Length::Fixed(23.0))
                .height(Length::Fixed(23.0)),
        )
        .width(Length::Fixed(TOOL_BUTTON_SIZE))
        .height(Length::Fixed(TOOL_BUTTON_SIZE))
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .padding(0)
    .width(Length::FillPortion(1))
    .height(Length::Fixed(TOOL_BUTTON_SIZE))
    .style(move |_, _| button_style(active, TOOL_BG, TOOL_ACTIVE_BG))
    .on_press(Message::ToggleModePressed)
    .into()
}

fn icon(icon: Icon, color: Color) -> svg::Svg<'static, Theme> {
    let handle = match icon {
        Icon::Reset => {
            svg::Handle::from_memory(&include_bytes!("../assets/icons/rotate-ccw.svg")[..])
        }
        Icon::Delete => svg::Handle::from_memory(&include_bytes!("../assets/icons/eraser.svg")[..]),
        Icon::Notes => {
            svg::Handle::from_memory(&include_bytes!("../assets/icons/notebook-pen.svg")[..])
        }
    };

    svg(handle).style(move |_, _| svg::Style { color: Some(color) })
}

fn build_keypad() -> Element<'static, Message> {
    let mut keypad = column![].spacing(10);

    for row_index in 0..3 {
        let mut line = row![].spacing(10);

        for col_index in 0..3 {
            let value = (row_index * 3 + col_index + 1) as u8;
            line = line.push(
                button(
                    container(
                        text(value.to_string())
                            .size(20)
                            .color(USER_TEXT_COLOR)
                            .width(Length::Fill)
                            .align_x(Horizontal::Center),
                    )
                    .width(Length::Fixed(DIGIT_BUTTON_SIZE))
                    .height(Length::Fixed(DIGIT_BUTTON_SIZE))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                )
                .padding(0)
                .width(Length::Fixed(DIGIT_BUTTON_SIZE))
                .height(Length::Fixed(DIGIT_BUTTON_SIZE))
                .style(|_, _| button_style(false, DIGIT_BG, DIGIT_ACTIVE_BG))
                .on_press(Message::DigitPressed(value)),
            );
        }

        keypad = keypad.push(line);
    }

    keypad.into()
}

fn primary_button(label: &'static str, message: Message) -> Element<'static, Message> {
    primary_button_enabled(label, message, true)
}

fn primary_button_enabled(
    label: &'static str,
    message: Message,
    enabled: bool,
) -> Element<'static, Message> {
    let text_color = if enabled {
        Color::WHITE
    } else {
        NOTE_TEXT_COLOR
    };
    let button = button(
        container(
            text(label)
                .size(18)
                .color(text_color)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .padding(0)
    .width(Length::Fill)
    .height(Length::Fixed(PRIMARY_BUTTON_HEIGHT))
    .style(|_, status| disabled_aware_button_style(status, true, TOOL_BG, TOOL_ACTIVE_BG));

    if enabled {
        button.on_press(message).into()
    } else {
        button.into()
    }
}

fn button_style(active: bool, base: Color, active_bg: Color) -> button::Style {
    button::Style {
        background: Some(Background::Color(if active { active_bg } else { base })),
        text_color: if active {
            Color::WHITE
        } else {
            USER_TEXT_COLOR
        },
        border: border::rounded(10).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    }
}

fn disabled_aware_button_style(
    status: button::Status,
    active: bool,
    base: Color,
    active_bg: Color,
) -> button::Style {
    if status == button::Status::Disabled {
        return button::Style {
            background: Some(Background::Color(PANEL_BG)),
            text_color: NOTE_TEXT_COLOR,
            border: border::rounded(10).width(0).color(Color::TRANSPARENT),
            ..Default::default()
        };
    }

    button_style(active, base, active_bg)
}

fn build_board(app: &SudokuApp) -> Element<'_, Message> {
    let mut board_column = column![separator_horizontal(THICK_LINE, board_width())];

    for block_row in 0..3 {
        let mut block_row_widget = row![separator_vertical(THICK_LINE, block_height())];

        for block_col in 0..3 {
            if block_col > 0 {
                block_row_widget =
                    block_row_widget.push(separator_vertical(THICK_LINE, block_height()));
            }

            block_row_widget = block_row_widget.push(build_box(app, block_row, block_col));
        }

        block_row_widget = block_row_widget.push(separator_vertical(THICK_LINE, block_height()));

        if block_row > 0 {
            board_column = board_column.push(separator_horizontal(THICK_LINE, board_width()));
        }

        board_column = board_column.push(block_row_widget);
    }

    board_column = board_column.push(separator_horizontal(THICK_LINE, board_width()));

    container(board_column)
        .style(|_| container::Style {
            background: Some(Background::Color(BOARD_BG)),
            ..Default::default()
        })
        .into()
}

fn build_box(app: &SudokuApp, block_row: usize, block_col: usize) -> Element<'_, Message> {
    let mut box_column = column![];

    for inner_row in 0..3 {
        let row_index = block_row * 3 + inner_row;
        let mut line = row![];

        for inner_col in 0..3 {
            let col_index = block_col * 3 + inner_col;

            if inner_col > 0 {
                line = line.push(separator_vertical(THIN_LINE, CELL_SIZE));
            }

            line = line.push(build_cell(app, row_index, col_index));
        }

        if inner_row > 0 {
            box_column = box_column.push(separator_horizontal(THIN_LINE, block_width()));
        }

        box_column = box_column.push(line);
    }

    container(box_column)
        .style(|_| container::Style {
            background: Some(Background::Color(BOARD_BG)),
            ..Default::default()
        })
        .into()
}

fn build_cell(app: &SudokuApp, row: usize, col: usize) -> Element<'_, Message> {
    let position = (row, col);
    let cell = app.board.cell(row, col);
    let selected = app.selected.contains(&position);
    let primary = primary_selection(&app.selected);
    let highlighted = primary
        .map(|(selected_row, selected_col)| selected_row == row || selected_col == col)
        .unwrap_or(false);
    let same_box = primary
        .map(|(selected_row, selected_col)| {
            selected_row / 3 == row / 3 && selected_col / 3 == col / 3
        })
        .unwrap_or(false);
    let selected_value = single_selected_value(app);
    let same_value = selected_value.is_some() && cell.value == selected_value;
    let invalid = should_show_conflicts(app) && app.board.has_conflict(row, col);

    let background = if selected {
        PRIMARY_BG
    } else if same_value {
        SAME_VALUE_BG
    } else if highlighted {
        HIGHLIGHT_BG
    } else if same_box {
        BOX_HIGHLIGHT_BG
    } else if invalid {
        INVALID_BG
    } else if cell.fixed {
        FIXED_BG
    } else {
        CELL_BG
    };

    let content = render_cell_content(cell, invalid, selected_value);

    button(
        container(content)
            .width(Length::Fixed(CELL_SIZE))
            .height(Length::Fixed(CELL_SIZE))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(move |_| container::Style {
                background: Some(Background::Color(background)),
                ..Default::default()
            }),
    )
    .padding(0)
    .width(Length::Fixed(CELL_SIZE))
    .height(Length::Fixed(CELL_SIZE))
    .style(|_, _| button::Style {
        background: None,
        border: border::rounded(0).width(0).color(Color::TRANSPARENT),
        ..Default::default()
    })
    .on_press(Message::CellPressed(row, col, app.modifiers))
    .into()
}

fn render_cell_content(
    cell: Cell,
    invalid: bool,
    selected_value: Option<u8>,
) -> Element<'static, Message> {
    if let Some(value) = cell.value {
        let color = if invalid {
            ERROR_TEXT_COLOR
        } else if cell.fixed {
            TEXT_COLOR
        } else {
            USER_TEXT_COLOR
        };

        container(
            text(value.to_string())
                .size(32)
                .color(color)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    } else {
        render_notes(cell, selected_value)
    }
}

fn render_notes(cell: Cell, selected_value: Option<u8>) -> Element<'static, Message> {
    let mut notes_column = column![].spacing(1);

    for note_row in 0..3 {
        let mut line = row![].spacing(2);

        for note_col in 0..3 {
            let note = (note_row * 3 + note_col + 1) as u8;
            let enabled = cell.notes[(note - 1) as usize];
            let matches_selected = enabled && selected_value == Some(note);
            let label = if enabled {
                note.to_string()
            } else {
                String::from(" ")
            };

            line = line.push(
                container(
                    text(label)
                        .size(12)
                        .color(if matches_selected {
                            ACTIVE_NOTE_TEXT_COLOR
                        } else {
                            NOTE_TEXT_COLOR
                        })
                        .font(Font {
                            weight: if matches_selected {
                                font::Weight::Bold
                            } else {
                                font::Weight::Normal
                            },
                            ..Font::default()
                        })
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            );
        }

        notes_column = notes_column.push(line);
    }

    container(notes_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([8, 6])
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

#[derive(Debug, Clone, Copy)]
enum Action {
    ToggleMode,
    SetDigit(u8),
    ClearCell,
    MoveSelection(i32, i32),
    Submit,
}

fn keyboard_action(event: iced::Event) -> Option<Action> {
    match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key.as_ref() {
            keyboard::Key::Named(key::Named::Space) => Some(Action::ToggleMode),
            keyboard::Key::Character("n") | keyboard::Key::Character("N") => {
                Some(Action::ToggleMode)
            }
            keyboard::Key::Character("1") => Some(Action::SetDigit(1)),
            keyboard::Key::Character("2") => Some(Action::SetDigit(2)),
            keyboard::Key::Character("3") => Some(Action::SetDigit(3)),
            keyboard::Key::Character("4") => Some(Action::SetDigit(4)),
            keyboard::Key::Character("5") => Some(Action::SetDigit(5)),
            keyboard::Key::Character("6") => Some(Action::SetDigit(6)),
            keyboard::Key::Character("7") => Some(Action::SetDigit(7)),
            keyboard::Key::Character("8") => Some(Action::SetDigit(8)),
            keyboard::Key::Character("9") => Some(Action::SetDigit(9)),
            keyboard::Key::Named(key::Named::Enter) => Some(Action::Submit),
            keyboard::Key::Named(key::Named::Backspace)
            | keyboard::Key::Named(key::Named::Delete) => Some(Action::ClearCell),
            keyboard::Key::Named(key::Named::ArrowUp) => Some(Action::MoveSelection(-1, 0)),
            keyboard::Key::Named(key::Named::ArrowDown) => Some(Action::MoveSelection(1, 0)),
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Action::MoveSelection(0, -1)),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Action::MoveSelection(0, 1)),
            _ => None,
        },
        _ => None,
    }
}

fn handle_action(app: &mut SudokuApp, action: Action) {
    if app.status == GameStatus::Finished {
        return;
    }

    match action {
        Action::ToggleMode => toggle_input_mode(app),
        Action::SetDigit(value) => apply_digit(app, value),
        Action::ClearCell => clear_selection(app),
        Action::MoveSelection(delta_row, delta_col) => {
            let (row, col) = primary_selection(&app.selected).unwrap_or((0, 0));
            let next_row = (row as i32 + delta_row).clamp(0, 8) as usize;
            let next_col = (col as i32 + delta_col).clamp(0, 8) as usize;

            app.selected.clear();
            app.selected.push((next_row, next_col));
        }
        Action::Submit => {
            if can_submit(app) {
                finish_game(app);
            }
        }
    }
}

fn reset_game(app: &mut SudokuApp, puzzle: Puzzle) {
    app.current_puzzle = puzzle;
    app.board = Board::from_start(puzzle);
    app.selected.clear();
    app.selected.push((0, 0));
    app.show_validation_result = false;
    app.mode = InputMode::Value;
    app.status = GameStatus::Playing;
    app.started_at = None;
    app.finished_elapsed = None;
}

fn start_game(app: &mut SudokuApp, puzzle: Puzzle) {
    reset_game(app, puzzle);
    app.started_at = Some(Instant::now());
}

fn finish_game(app: &mut SudokuApp) {
    app.show_validation_result = true;
    app.status = GameStatus::Finished;
    app.finished_elapsed = Some(
        app.started_at
            .map(|started_at| started_at.elapsed())
            .unwrap_or_default(),
    );
}

fn clear_selection(app: &mut SudokuApp) {
    if !can_clear_selection(app) {
        return;
    }

    if app.mode == InputMode::Notes {
        for &(row, col) in &app.selected {
            app.board.clear_notes(row, col);
        }
    } else if let Some((row, col)) = primary_selection(&app.selected) {
        app.board.clear_value(row, col);
    }

    app.show_validation_result = false;
}

fn toggle_input_mode(app: &mut SudokuApp) {
    app.mode = app.mode.toggle();
    if app.mode == InputMode::Value {
        collapse_to_primary_selection(&mut app.selected);
    }
}

fn apply_digit(app: &mut SudokuApp, value: u8) {
    if app.mode == InputMode::Notes {
        for &(row, col) in &app.selected {
            app.board.toggle_note(row, col, value);
        }
    } else if let Some((row, col)) = primary_selection(&app.selected) {
        if app.board.set_cell(row, col, Some(value)) {
            app.board.remove_related_notes(row, col, value);
        }
    }
    app.show_validation_result = false;
}

fn can_submit(app: &SudokuApp) -> bool {
    app.status == GameStatus::Playing
        && app.validation_mode == ValidationMode::Manual
        && app.board.is_filled()
}

fn can_clear_selection(app: &SudokuApp) -> bool {
    if app.status == GameStatus::Finished {
        return false;
    }

    primary_selection(&app.selected)
        .map(|(row, col)| !app.board.cell(row, col).fixed)
        .unwrap_or(false)
}

fn filled_count(board: &Board) -> usize {
    let mut count = 0;

    for row in 0..9 {
        for col in 0..9 {
            if board.cell(row, col).value.is_some() {
                count += 1;
            }
        }
    }

    count
}

fn error_count(board: &Board) -> usize {
    let mut count = 0;

    for row in 0..9 {
        for col in 0..9 {
            if board.cell(row, col).value.is_some() && board.has_conflict(row, col) {
                count += 1;
            }
        }
    }

    count
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{minutes}:{seconds:02}")
}

fn should_show_conflicts(app: &SudokuApp) -> bool {
    (app.status == GameStatus::Finished || app.mode == InputMode::Value)
        && match app.validation_mode {
            ValidationMode::Live => true,
            ValidationMode::Manual => app.show_validation_result,
        }
}

fn single_selected_value(app: &SudokuApp) -> Option<u8> {
    if app.selected.len() == 1 {
        primary_selection(&app.selected).and_then(|(selected_row, selected_col)| {
            app.board.cell(selected_row, selected_col).value
        })
    } else {
        None
    }
}

fn primary_selection(selected: &[Position]) -> Option<Position> {
    selected.last().copied()
}

fn collapse_to_primary_selection(selected: &mut Vec<Position>) {
    if let Some(primary) = primary_selection(selected) {
        selected.clear();
        selected.push(primary);
    }
}

fn select_range(
    selected: &mut Vec<Position>,
    start: Option<Position>,
    end: Position,
    board: &Board,
) {
    let start = start.unwrap_or(end);
    let row_start = start.0.min(end.0);
    let row_end = start.0.max(end.0);
    let col_start = start.1.min(end.1);
    let col_end = start.1.max(end.1);

    selected.clear();

    for row in row_start..=row_end {
        for col in col_start..=col_end {
            if !board.cell(row, col).fixed {
                selected.push((row, col));
            }
        }
    }

    if selected.is_empty() {
        selected.push(end);
    }
}

fn separator_vertical(width: f32, height: f32) -> Element<'static, Message> {
    container("")
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(|_| container::Style {
            background: Some(Background::Color(LINE_COLOR)),
            ..Default::default()
        })
        .into()
}

fn separator_horizontal(height: f32, width: f32) -> Element<'static, Message> {
    container("")
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(|_| container::Style {
            background: Some(Background::Color(LINE_COLOR)),
            ..Default::default()
        })
        .into()
}

fn block_width() -> f32 {
    CELL_SIZE * 3.0 + THIN_LINE * 2.0
}

fn block_height() -> f32 {
    CELL_SIZE * 3.0 + THIN_LINE * 2.0
}

fn board_width() -> f32 {
    block_width() * 3.0 + THICK_LINE * 4.0
}

fn board_height() -> f32 {
    block_height() * 3.0 + THICK_LINE * 4.0
}
