const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

struct Paddle {
    rect: ggez::graphics::Rect,
    vel: f32,
}

struct Ball {
    rect: ggez::graphics::Rect,
    vel: ggez::mint::Vector2<f32>,
}

struct MainState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let player1 = Paddle {
            rect: ggez::graphics::Rect::new(30.0, (WINDOW_HEIGHT - 150.0) / 2.0, 20.0, 150.0),
            vel: 400.0,
        };
        let player2 = Paddle {
            rect: ggez::graphics::Rect::new(
                WINDOW_WIDTH - 50.0,
                (WINDOW_HEIGHT - 150.0) / 2.0,
                20.0,
                150.0,
            ),
            vel: 400.0,
        };
        let ball = Ball {
            rect: ggez::graphics::Rect::new(
                WINDOW_WIDTH / 2.0 - 10.0,
                WINDOW_HEIGHT / 2.0 - 10.0,
                20.0,
                20.0,
            ),
            vel: ggez::mint::Vector2 {
                x: -300.0,
                y: 200.0,
            },
        };

        Ok(MainState {
            player1,
            player2,
            ball,
        })
    }

    fn update_paddles(&mut self, ctx: &mut ggez::Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        fn clamp(value: f32, min: f32, max: f32) -> f32 {
            value.max(min).min(max)
        }

        let move_paddle = |paddle: &mut Paddle, dy: f32| {
            paddle.rect.y = clamp(paddle.rect.y + dy, 0.0, WINDOW_HEIGHT - paddle.rect.h);
        };

        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::W) {
            let dy = -self.player1.vel * dt;
            move_paddle(&mut self.player1, dy);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::S) {
            let dy = self.player1.vel * dt;
            move_paddle(&mut self.player1, dy);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Up) {
            let dy = -self.player2.vel * dt;
            move_paddle(&mut self.player2, dy);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Down) {
            let dy = self.player2.vel * dt;
            move_paddle(&mut self.player2, dy);
        }
    }

    fn update_ball(&mut self, ctx: &mut ggez::Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.ball.rect.x += self.ball.vel.x * dt;
        self.ball.rect.y += self.ball.vel.y * dt;

        // Überprüfe Kollisionen mit den Schlägern
        let collides_with_p1 = self.ball.rect.overlaps(&self.player1.rect) && self.ball.vel.x < 0.0;
        let collides_with_p2 = self.ball.rect.overlaps(&self.player2.rect) && self.ball.vel.x > 0.0;

        if collides_with_p1 || collides_with_p2 {
            self.ball.vel.x = -self.ball.vel.x;
        }

        // Überprüfe Kollisionen mit dem oberen und unteren Bildschirmrand
        let collide_with_lower_screen = self.ball.rect.y <= 0.0 && self.ball.vel.y < 0.0;
        let collide_with_upper_screen =
            self.ball.rect.y + self.ball.rect.h >= WINDOW_HEIGHT && self.ball.vel.y > 0.0;

        if collide_with_lower_screen || collide_with_upper_screen {
            self.ball.vel.y = -self.ball.vel.y;
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for MainState {
    // Update-Funktion für Spiellogik
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Aktualisiere Schlägerpositionen, Ballposition und Kollisionen
        self.update_paddles(ctx);
        self.update_ball(ctx);
        Ok(())
    }

    // Draw-Funktion für die Grafikdarstellung
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Lösche den Bildschirm
        ggez::graphics::clear(ctx, ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.0));

        // Zeichne die Schläger
        let paddle_color = ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        draw_rect(ctx, &self.player1.rect, paddle_color)?;
        draw_rect(ctx, &self.player2.rect, paddle_color)?;

        // Zeichne den Ball
        let ball_color = ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        draw_rect(ctx, &self.ball.rect, ball_color)?;

        // Präsentiere die Grafiken
        ggez::graphics::present(ctx)?;

        // Schlafzeit, um den CPU-Verbrauch zu reduzieren
        ggez::timer::yield_now();

        Ok(())
    }
}

fn draw_rect(
    ctx: &mut ggez::Context,
    rect: &ggez::graphics::Rect,
    color: ggez::graphics::Color,
) -> ggez::GameResult {
    let mesh =
        ggez::graphics::Mesh::new_rectangle(ctx, ggez::graphics::DrawMode::fill(), *rect, color)?;

    ggez::graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())
}

fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Oliver Koch");
    let (ctx, events_loop) = cb.build()?;
    let state = MainState::new()?;
    ggez::event::run(ctx, events_loop, state)
}
