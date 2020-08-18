use crate::physics::body::Body;
use crate::physics::collision;
use crate::Polygon;
use std::collections::HashMap;

const COLLISION_LOOP_TRIES: u32 = 100;

pub struct World {
    pub bodies: HashMap<u32, Body>,
    pub current_id: u32,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn add_body(
        &mut self,
        shape: Polygon,
        mass: f64,
        friction: f64,
        frictionless: bool,
        gravityless: bool,
    ) -> u32 {
        self.current_id += 1;
        self.bodies.insert(
            self.current_id,
            Body::new(shape, mass, friction, frictionless, gravityless),
        );
        self.current_id
    }

    pub fn add_static_body(&mut self, shape: Polygon, friction: f64) -> u32 {
        self.add_body(shape, f64::INFINITY, friction, false, true)
    }

    pub fn body(&mut self, id: u32) -> Option<&mut Body> {
        self.bodies.get_mut(&id)
    }

    pub fn update(&mut self, delta: f64) {
        // separate_bodies()

        for (_, body) in &mut self.bodies {
            // Gravity
            if !body.gravityless {
                body.velocity.y -= 4.0;
            }
        }

        // Main loop
        let mut tries = COLLISION_LOOP_TRIES;
        let mut time_left = delta;

        while time_left >= 1e-3 && tries >= 0 {
            // Find earliest collision
            let mut earliest_collision_time = None;
            let mut earliest_collision_normal = None;
            let mut earliest_collision_body_a_id = None;
            let mut earliest_collision_body_b_id = None;

            for (id_a, a) in &self.bodies {
                for (id_b, b) in &self.bodies {
                    if id_a == id_b {
                        continue;
                    }
                    if let Some((collision_time, collision_normal)) =
                        collision::get_collision_time_and_normal(
                            &a.shape, a.velocity, &b.shape, b.velocity,
                        )
                    {
                        if collision_time <= time_left {
                            if earliest_collision_time.is_none()
                                || collision_time <= earliest_collision_time.unwrap()
                            {
                                earliest_collision_time = Some(collision_time);
                                earliest_collision_normal = Some(collision_normal);
                                earliest_collision_body_a_id = Some(*id_a);
                                earliest_collision_body_b_id = Some(*id_b);
                            }
                        }
                    }
                }
            }

            // Goto earliest collision
            let mut elapsed_time;
            if let Some(earliest_collision_time) = earliest_collision_time {
                if earliest_collision_time <= 1e-10 {
                    elapsed_time = 0.0;
                } else {
                    elapsed_time = earliest_collision_time - 1e-10;
                }
            } else {
                elapsed_time = time_left;
            }

            for (_, body) in &mut self.bodies {
                body.move_xy(body.velocity * elapsed_time);
            }

            /*
            // Collision Response
            if let(earliest_collision_time) = earliest_collision_time {
                if let(earliest_collision_normal) = earliest_collision_normal {

                    // Renaming
                    let a = self.bodies.get(&earliest_collision_body_a_id.unwrap());
                    let b = self.bodies.get(&earliest_collision_body_b_id.unwrap());


                    // Decomposition
                    a_velocity_into_plane, a_velocity_along_plane = a.velocity.decompose_into(earliest_collision_normal)
                    b_velocity_into_plane, b_velocity_along_plane = b.velocity.decompose_into(earliest_collision_normal)

                    // Velocity into plane
                    a_mass_ratio = get_ratio(a.mass, b.mass)
                    new_velocity_into_plane = a_velocity_into_plane * a_mass_ratio + b_velocity_into_plane * (1.0 - a_mass_ratio)

                    // Velocity along plane (friction)
                    velocity_keep = a.frictionless || b.frictionless ? 1.0 : (1.0 - a.friction) * (1.0 - b.friction)
                    a_new_velocity_along_plane = a_velocity_along_plane * velocity_keep
                    b_new_velocity_along_plane = b_velocity_along_plane * velocity_keep


                    // Apply velocity
                    a.velocity = a_new_velocity_along_plane + new_velocity_into_plane
                    b.velocity = b_new_velocity_along_plane + new_velocity_into_plane

                }
            }
            */

            // Advance cycle
            //time_left -= elapsed_time
            //tries-= 1
        }

        /*
        // If there's still time left, we ignore collisions and just attempt to finish the update
        @bodies.each do |body|
          body.move(body.velocity * time_left)
        end
        separate_bodies()
        */
    }
}

/*

    private

    def separate_bodies()
      each_collidable_bodies do |a, b|
        if Nulu::Collision.colliding?(a.shape, b.shape)
          mtv = Nulu::Collision.mtv(a.shape, b.shape)
          a_mass_ratio = get_ratio(a.mass, b.mass)
          b.move(mtv * a_mass_ratio)
          a.move(-mtv * (1.0 - a_mass_ratio))
        end
      end
    end

    def each_collidable_bodies()
      @collision_enabler.each_collidable_pair do |a, b|
        yield(a, b) unless @collision_skip.include?(SortedPair.new(a.id, b.id))
      end
    end

    def get_ratio(a, b)
      if a == b
        return 0.5
      elsif a == INF
        return 1.0
      elsif b == INF
        return 0.0
      else
        return a / (a + b)
      end
    end

    class SortedPair
      attr_reader :min, :max
      def initialize(a, b) @min, @max = [a,b].min, [a,b].max end
      def eql?(other) min.eql?(other.min) && max.eql?(other.max) end
      def hash() [min, max].hash end
    end

  end
end

*/
