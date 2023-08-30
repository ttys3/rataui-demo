use std::io;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarState, ScrollbarOrientation, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

const PAGE_SIZE: usize = 5;

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

pub fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {

    // Initialize conversation state and pagination
    let mut conversations = fetch_conversations(0); // Fetch the first page of conversations
    let mut selected_index = 0;
    let mut current_page = 0;

    let mut app = App::default();

    loop {
        // Handle user input events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break, // Quit the program when 'q' is pressed

                    KeyCode::Char('j') => {
                        app.vertical_scroll = app.vertical_scroll.saturating_add(10);
                        app.vertical_scroll_state = app
                            .vertical_scroll_state
                            .position(app.vertical_scroll as u16);
                    }
                    KeyCode::Char('k') => {
                        app.vertical_scroll = app.vertical_scroll.saturating_sub(10);
                        app.vertical_scroll_state = app
                            .vertical_scroll_state
                            .position(app.vertical_scroll as u16);
                    }
                    KeyCode::Char('h') => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_sub(10);
                        app.horizontal_scroll_state = app
                            .horizontal_scroll_state
                            .position(app.horizontal_scroll as u16);
                    }
                    KeyCode::Char('l') => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_add(10);
                        app.horizontal_scroll_state = app
                            .horizontal_scroll_state
                            .position(app.horizontal_scroll as u16);
                    }

                    KeyCode::Down => {
                        // Move selection down
                        selected_index = (selected_index + 1).min(conversations.len() - 1);
                    }
                    KeyCode::Up => {
                        // Move selection up
                        selected_index = selected_index.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        // Fetch next page of conversations
                        current_page += 1;
                        conversations = fetch_conversations(current_page);
                        selected_index = 0; // Reset the selected index to the first conversation
                    }
                    KeyCode::PageUp => {
                        // Fetch previous page of conversations
                        if current_page > 0 {
                            current_page -= 1;
                            conversations = fetch_conversations(current_page);
                            selected_index = 0; // Reset the selected index to the first conversation
                        }
                    }
                    _ => {}
                }
            }
        }

        // Prepare UI layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(8), Constraint::Max(3)].as_ref())
            .split(terminal.size()?);

        // Prepare conversation list widget
        let conversation_items: Vec<ListItem> = conversations
            .iter()
            .map(|conv| {
                ListItem::new(format!("Title: {} ID: {}", conv.title, conv.id))
            })
            .collect();
        let conversations_list = List::new(conversation_items)
            .block(Block::default().title("Conversations").borders(Borders::ALL));


        let bottom_cmdbar = Paragraph::new("History    |     Config    |     History    |     New    |     Shared")
            .block(Block::default().title("Cmd: ").borders(Borders::ALL));

        // Render UI
        terminal.draw(|f| {


            // Prepare conversation details widget
            let selected_conversation = &conversations[selected_index];
            let conversation_details = Paragraph::new(selected_conversation.details())
                .block(Block::default().title("Conversation Details").borders(Borders::ALL))
                .wrap(Wrap { trim: true })
                .scroll((app.vertical_scroll as u16, 0));

            app.vertical_scroll_state = app.vertical_scroll_state.content_length(selected_conversation.details().len() as u16);
            app.horizontal_scroll_state = app
                .horizontal_scroll_state
                .content_length(1024 as u16);

            f.render_widget(conversations_list, chunks[0]);
            f.render_widget(conversation_details, chunks[1]);
            f.render_widget(bottom_cmdbar, chunks[2]);

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                chunks[1],
                &mut app.vertical_scroll_state,
            );
        })?;
    }

    Ok(())
}

// Struct representing a conversation
struct Conversation {
    title: String,
    id: String,
    // Add more fields as needed
}

impl Conversation {
    // Replace this method with your API call to fetch conversation details
    fn details(&self) -> String {
        // Simulating conversation details for demonstration purposes
        let details = r#"""
        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        Some details about the conversation...

        ----------------------------------------------------------------------------------------------------------------------------------------------------->
        """#.to_owned();
        format!(
            "Conversation Title: {}\nConversation ID: {}\n\nConversation Details:\n{}",
            self.title, self.id, details
        )
    }
}

// Function to fetch conversations (replace with your API call)
fn fetch_conversations(page: usize) -> Vec<Conversation> {
    // Simulating fetching conversations for demonstration purposes
    let start_index = page * PAGE_SIZE;
    let end_index = start_index + PAGE_SIZE;

    (start_index..end_index)
        .map(|i| Conversation {
            title: format!("Conversation {}", i + 1),
            id: format!("conv-{}", i + 1),
        })
        .collect()
}
