#[derive(Clone, Debug)]
pub enum CanvasRPC {
    // todo: this should be taken into the tx<>, not here
    Quit,
    Init(u32),

    Save,
    Restore,
    BeginPath,
    ClosePath,
    Stroke,
    Fill,

    Scale(f64, f64),
    Resize(f64, f64),
    MoveTo(f64, f64),
    LineTo(f64, f64),

    ShadowBlur(f64),
    LineWidth(f64),

    FillRect(f64, f64, f64, f64),
    Rect(f64, f64, f64, f64),
    ClearRect(f64, f64, f64, f64),

    Arc(f64, f64, f64, f64, f64, bool),
    BezierCurveTo(f64, f64, f64, f64, f64, f64),
    QuadraticCurveTo(f64, f64, f64, f64),

    FillText(String, f64, f64),

    FillStyle(String),
    BackgroundFillStyle(String),
    StrokeStyle(String),
    Font(String),
    ShadowColor(String),
    LineCap(String),

    LineDash(Vec<f64>),
}