Implementations
Source
impl Term
Source
pub fn stdout() -> Term ⓘ

Return a new unbuffered terminal.
Source
pub fn stderr() -> Term ⓘ

Return a new unbuffered terminal to stderr.
Source
pub fn buffered_stdout() -> Term ⓘ

Return a new buffered terminal.
Source
pub fn buffered_stderr() -> Term ⓘ

Return a new buffered terminal to stderr.
Source
pub fn read_write_pair<R, W>(read: R, write: W) -> Term ⓘ
where
    R: Read + Debug + AsRawFd + Send + 'static,
    W: Write + Debug + AsRawFd + Send + 'static,

Return a terminal for the given Read/Write pair styled like stderr.
Source
pub fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term ⓘ
where
    R: Read + Debug + AsRawFd + Send + 'static,
    W: Write + Debug + AsRawFd + Send + 'static,

Return a terminal for the given Read/Write pair.
Source
pub fn style(&self) -> Style

Return the style for this terminal.
Source
pub fn target(&self) -> TermTarget

Return the target of this terminal.
Source
pub fn write_line(&self, s: &str) -> Result<()>

Write a string to the terminal and add a newline.
Source
pub fn read_char(&self) -> Result<char>

Read a single character from the terminal.

This does not echo the character and blocks until a single character or complete key chord is entered. If the terminal is not user attended the return value will be an error.
Source
pub fn read_key(&self) -> Result<Key>

Read a single key form the terminal.

This does not echo anything. If the terminal is not user attended the return value will always be the unknown key.
Source
pub fn read_key_raw(&self) -> Result<Key>
Source
pub fn read_line(&self) -> Result<String>

Read one line of input.

This does not include the trailing newline. If the terminal is not user attended the return value will always be an empty string.
Source
pub fn read_line_initial_text(&self, initial: &str) -> Result<String>

Read one line of input with initial text.

This method blocks until no other thread is waiting for this read_line before reading a line from the terminal. This does not include the trailing newline. If the terminal is not user attended the return value will always be an empty string.
Source
pub fn read_secure_line(&self) -> Result<String>

Read a line of input securely.

This is similar to read_line but will not echo the output. This also switches the terminal into a different mode where not all characters might be accepted.
Source
pub fn flush(&self) -> Result<()>

Flush internal buffers.

This forces the contents of the internal buffer to be written to the terminal. This is unnecessary for unbuffered terminals which will automatically flush.
Source
pub fn is_term(&self) -> bool

Check if the terminal is indeed a terminal.
Source
pub fn features(&self) -> TermFeatures<'_>

Check for common terminal features.
Source
pub fn size(&self) -> (u16, u16)

Return the terminal size in rows and columns or gets sensible defaults.
Source
pub fn size_checked(&self) -> Option<(u16, u16)>

Return the terminal size in rows and columns.

If the size cannot be reliably determined None is returned.
Source
pub fn move_cursor_to(&self, x: usize, y: usize) -> Result<()>

Move the cursor to row x and column y. Values are 0-based.
Source
pub fn move_cursor_up(&self, n: usize) -> Result<()>

Move the cursor up by n lines, if possible.

If there are less than n lines above the current cursor position, the cursor is moved to the top line of the terminal (i.e., as far up as possible).
Source
pub fn move_cursor_down(&self, n: usize) -> Result<()>

Move the cursor down by n lines, if possible.

If there are less than n lines below the current cursor position, the cursor is moved to the bottom line of the terminal (i.e., as far down as possible).
Source
pub fn move_cursor_left(&self, n: usize) -> Result<()>

Move the cursor n characters to the left, if possible.

If there are fewer than n characters to the left of the current cursor position, the cursor is moved to the beginning of the line (i.e., as far to the left as possible).
Source
pub fn move_cursor_right(&self, n: usize) -> Result<()>

Move the cursor n characters to the right.

If there are fewer than n characters to the right of the current cursor position, the cursor is moved to the end of the current line (i.e., as far to the right as possible).
Source
pub fn clear_line(&self) -> Result<()>

Clear the current line.

Position the cursor at the beginning of the current line.
Source
pub fn clear_last_lines(&self, n: usize) -> Result<()>

Clear the last n lines before the current line.

Position the cursor at the beginning of the first line that was cleared.
Source
pub fn clear_screen(&self) -> Result<()>

Clear the entire screen.

Move the cursor to the upper left corner of the screen.
Source
pub fn clear_to_end_of_screen(&self) -> Result<()>

Clear everything from the current cursor position to the end of the screen. The cursor stays in its position.
Source
pub fn clear_chars(&self, n: usize) -> Result<()>

Clear the last n characters of the current line.
Source
pub fn set_title<T: Display>(&self, title: T)

Set the terminal title.
Source
pub fn show_cursor(&self) -> Result<()>

Make the cursor visible again.
Source
pub fn hide_cursor(&self) -> Result<()>

Hide the cursor.
