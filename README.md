# Click-R: Automatic Clicker

Click-R is a Rust-based application that allows you to automate mouse clicks with a specified interval and number of
clicks. The application uses the `iced` library for the graphical interface and `enigo` for mouse control.

## Features

- **Interval Setup:** Set the interval between clicks in seconds.
- **Click Count:** Set the number of clicks per cycle.
- **Mouse Button Selection:** Choose the mouse button for automatic clicks (left, right, middle).
- **Theme:** Select the interface theme (light or dark).
- **Start and Stop:** Control the automatic clicking process with "Start" and "Stop" buttons.

## Installation

1. Ensure you have Rust and Cargo installed. If not, install them from
   the [official Rust website](https://www.rust-lang.org/).

2. Clone the repository:

   ```sh
   git clone https://github.com/your-repo/click-r.git
   cd click-r
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

4. Run the application:

   ```sh
   cargo run --release
   ```

___

## Usage

- **Theme Selection**: Use the dropdown list to select the interface theme.
- **Interval Setup**: Use the slider to set the interval between clicks.
- **Click Count Setup**: Use the slider to set the number of clicks per cycle.
- **Mouse Button Selection**: Click on the button corresponding to the desired mouse button (left, right, middle).
- **Start and Stop**: Press "Start" to begin automatic clicks and "Stop" to stop them.

___

## License

This project is licensed under the MIT License.

___

## Acknowledgments

- **iced**: A library for creating a graphical interface.
- **enigo**: A library for keyboard and mouse control.

___

## Contact

If you have any questions or suggestions, please contact the project author.

___

Enjoy using Click-R and automate your mouse clicks with ease!
