use nannou::prelude::*;

const WIN_WIDTH: f32 = 1440.0;
const WIN_HEIGHT: f32 = 810.0;
const HALF_WIDTH: f32 = WIN_WIDTH / 2.0;
const HALF_HEIGHT: f32 = WIN_HEIGHT / 2.0;
const NUM_AGENTS: usize = 99;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    agents: [Agent; NUM_AGENTS],
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let mut agents: Vec<Agent> = Vec::with_capacity(NUM_AGENTS);
    for i in 0..NUM_AGENTS {
        let mut max_radius = 63.0;
        if random::<f32>() < 0.045{ max_radius = max_radius * 4.5};
        agents.push(Agent {
            pos: Vec2::new(random_range(-HALF_WIDTH, HALF_WIDTH), random_range(-HALF_HEIGHT, HALF_HEIGHT)),
            radius: random_range(5.0, max_radius),
            // speed: random_range(0.45, 1.8),
            speed:  ((i as f32)+1.0) / 45.0,
            color: hsl(random_range(0.63, 0.81), 0.72, i as f32 / 300.0)
        });
    }
    Model {
        _window,
        agents: agents.try_into().unwrap(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.agents.iter_mut().enumerate().for_each(|(_i, x)| x.update());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for agent in &model.agents {
        agent.render(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
#[derive(Debug)]
struct Agent {
    pos: Vec2,
    radius: f32,
    speed: f32,
    color: Hsl
}

impl Agent {
    fn render(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .radius(self.radius)
            .color(self.color);
    }

    fn update(&mut self) {
        self.pos.x = self.pos.x + self.speed;
        if self.pos.x > HALF_WIDTH + self.radius {
            self.pos.x = -(HALF_WIDTH + self.radius);
            self.pos.y = random_range(-HALF_HEIGHT, HALF_HEIGHT);
        }
    }
}
