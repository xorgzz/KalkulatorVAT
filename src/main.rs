use slint::Weak;

slint::include_modules!();

const VAT:f64 = 0.23;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
	let ui_h: Weak<AppWindow> = ui.as_weak();
	ui.on_calculate_VAT(move |string| {
		let ui: AppWindow = ui_h.unwrap();
		let num: f64 = string.trim().parse().unwrap();
		let tax: f64 = num*VAT;
		let output: String = format!("Netto: {:.2}zł\nPodatek VAT: {:.2}zł", num - tax, tax);
		ui.set_out(output.into());
	});

    ui.run()
}
