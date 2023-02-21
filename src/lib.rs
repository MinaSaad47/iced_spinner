use std::f64::consts::FRAC_PI_2;
use std::time::{Duration, Instant};
use iced::Size;
use iced_native::{Clipboard, Color, Element, Event, Length, Point, Rectangle, Shell, window};
use iced_native::{Layout, renderer, Widget};
use iced_native::event::Status;
use iced_native::layout::{Limits, Node};
use iced_native::renderer::Style;
use iced_native::widget::Tree;
use iced_native::widget::tree::{State, Tag};

pub struct Spinner {
    width: Length,
    height: Length,
    rate: Duration,
    padding: f32,
    radius: f32,
    spread: f32,
    count: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            rate: Duration::from_secs_f32(1.0),
            padding: 4.0,
            radius: 2.0,
            count: 1,
            spread: std::f32::consts::PI * 2.0,
        }
    }

    pub fn rate(mut self, rate: Duration) -> Self {
        self.rate = rate;
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

struct SpinnerState {
    last_update: Instant,
    t: f32,
}

impl<Message, Renderer> Widget<Message, Renderer> for Spinner
    where
        Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.width(self.width).height(self.height).resolve(Size::new(f32::INFINITY, f32::INFINITY)))
    }

    fn draw(&self, state: &Tree, renderer: &mut Renderer, theme: &Renderer::Theme, style: &Style, layout: Layout<'_>, cursor_position: Point, viewport: &Rectangle) {
        let bounds = layout.bounds();
        let size = if bounds.width < bounds.height { bounds.width } else { bounds.height } / 2.0;
        let state = state.state.downcast_ref::<SpinnerState>();
        let center = bounds.center();
        let distance_from_center = size / 2.0;

        for i in 0..self.count {
            let t = state.t + i as f32 * self.spread / self.count as f32;
            let (y, x) = (t * std::f32::consts::PI * 2.0).sin_cos();

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: center.x + x * distance_from_center - self.radius,
                        y: center.y + y * distance_from_center - self.radius,
                        width: self.radius * 2.0,
                        height: self.radius * 2.0,
                    },
                    border_radius: self.radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Color::BLACK,
            );
        }
    }

    fn tag(&self) -> Tag {
        Tag::of::<SpinnerState>()
    }

    fn state(&self) -> State {
        State::new(SpinnerState{
            last_update: Instant::now(),
            t: 0.0,
        })
    }

    fn on_event(&mut self, state: &mut Tree, event: Event, _layout: Layout<'_>, _cursor_position: Point, _renderer: &Renderer, _clipboard: &mut dyn Clipboard, shell: &mut Shell<'_, Message>) -> Status {
        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = state.state.downcast_mut::<SpinnerState>();
                let duration = (now - state.last_update).as_secs_f32();

                state.t += duration * 1.0 / self.rate.as_secs_f32();
                if state.t > 1.0 {
                    state.t -= 1.0;
                }

                shell.request_redraw(window::RedrawRequest::At(
                    now + Duration::from_millis(1000 / 60),
                ));
                state.last_update = now;
                return Status::Captured
            }
            _ => ()
        }

        Status::Ignored
    }
}

impl<'a, Message, Renderer> From<Spinner> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
{
    fn from(spinner: Spinner) -> Self {
        Self::new(spinner)
    }
}

pub fn spinner() -> Spinner {
    Spinner::new()
}