//! Structures for defining geometry primitives.
//!
//! TODO move hittesting to a separate crate?

use crate::math::*;
use rand;
use std::fmt;

pub struct Interval {
  pub min: f32,
  pub max: f32,
}

impl Interval {
  pub fn new(min: f32, max: f32) -> Interval {
    Interval { min, max }
  }
}

#[derive(Default, Copy, Clone)]
pub struct Circle {
  pub center: Vec2,
  pub r: f32,
}

#[derive(Default, Copy, Clone)]
pub struct Rect {
  pub min_x: f32,
  pub min_y: f32,
  pub max_x: f32,
  pub max_y: f32,
}

#[derive(Default, Copy, Clone)]
pub struct LineSegment {
  pub a: Vec2,
  pub b: Vec2,
  pub normal: Vec2,
}

impl LineSegment {
  pub fn new(a: Vec2, b: Vec2, normal: Vec2) -> LineSegment {
    LineSegment { a, b, normal }
  }
}

#[derive(Default, Copy, Clone)]
pub struct OverlapResult {
  pub normal: Vec2,
  pub distance: f32,
}

#[derive(Default, Copy, Clone)]
pub struct SweepResult {
  pub normal: Vec2,
  pub t: f32,
}

#[derive(Copy, Clone)]
pub enum Shape {
  Point(Vec2),
  Circle(Circle),
  Rect(Rect),
}

impl Default for Shape {
  fn default() -> Self {
    Shape::Point(Vec2::zero())
  }
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Shape::Point(p) => write!(f, "Point ({}, {})", p.x(), p.y()),
      Shape::Circle(c) => write!(
        f,
        "Circle (r: {}, center: {}, {})",
        c.r,
        c.center.x(),
        c.center.y()
      ),
      Shape::Rect(r) => write!(
        f,
        "Rect ({}, {}, {}, {})",
        r.min_x, r.max_x, r.min_y, r.max_y
      ),
    }
  }
}

pub struct ProjectionResult {
  pub step: Vec2,
  pub step_mag: f32,
  pub collision_normal: Vec2,
}

///////////////////////////////////////////////////////////////////////////////

impl Rect {
  pub fn left_edge(self) -> LineSegment {
    let a: Vec2 = vec2(self.min_x, self.min_y);
    let b: Vec2 = vec2(self.min_x, self.max_y);
    return LineSegment::new(a, b, left());
  }

  pub fn bottom_edge(self) -> LineSegment {
    let a: Vec2 = vec2(self.min_x, self.min_y);
    let b: Vec2 = vec2(self.max_x, self.min_y);
    return LineSegment::new(a, b, down());
  }

  pub fn top_edge(self) -> LineSegment {
    let a: Vec2 = vec2(self.min_x, self.max_y);
    let b: Vec2 = vec2(self.max_x, self.max_y);
    return LineSegment::new(a, b, up());
  }

  pub fn right_edge(self: Rect) -> LineSegment {
    let a: Vec2 = vec2(self.max_x, self.max_y);
    let b: Vec2 = vec2(self.max_x, self.min_y);
    return LineSegment::new(a, b, right());
  }

  pub fn w(self) -> f32 {
    self.max_x - self.min_x
  }
  pub fn h(self) -> f32 {
    self.max_y - self.min_y
  }
}

///////////////////////////////////////////////////////////////////////////////

fn left() -> Vec2 {
  vec2(-1.0, 0.0)
}

fn right() -> Vec2 {
  Vec2::unit_x()
}

fn down() -> Vec2 {
  vec2(0.0, -1.0)
}

fn up() -> Vec2 {
  Vec2::unit_y()
}

///////////////////////////////////////////////////////////////////////////////

// TODO handle with Option<Vec2> ?
pub fn get_line_intersection(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, i: &mut Vec2) -> bool {
  // shamelessly borrowed from https://stackoverflow.com/a/1968345

  let s1: Vec2 = p1 - p0;
  let s2: Vec2 = p3 - p2;

  let s = (-s1.y() * (p0.x() - p2.x()) + s1.x() * (p0.y() - p2.y()))
    / (-s2.x() * s1.y() + s1.x() * s2.y());
  let t = (s2.x() * (p0.y() - p2.y()) - s2.y() * (p0.x() - p2.x()))
    / (-s2.x() * s1.y() + s1.x() * s2.y());

  if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
    // TODO rewrite this to be more idiomatic in Rust
    // Collision detected
    i.set_x(p0.x() + (t * s1.x()));
    i.set_y(p0.y() + (t * s1.y()));
    return true;
  } else {
    // No collision
    return false;
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn project_circle_v_segment(result: &mut ProjectionResult, c: Circle, edge: LineSegment) {
  // use the edge's normal to find the nearest point on the circle, this
  // is the earliest possible point of intersection, and therefore the
  // only one we care about.
  let circ_edge_a: Vec2 = c.center - (edge.normal * c.r);
  let circ_edge_b: Vec2 = circ_edge_a + result.step;

  let mut intersection: Vec2 = Vec2::zero();
  let intersection_exists =
    get_line_intersection(edge.a, edge.b, circ_edge_a, circ_edge_b, &mut intersection);
  if intersection_exists {
    let hit_dist: Vec2 = intersection - circ_edge_a;
    let hit_magnitude = hit_dist.length();
    if hit_magnitude < result.step_mag {
      result.step_mag = hit_magnitude;
      result.step = (result.step.normalize()) * hit_magnitude;
      result.collision_normal = edge.normal;
    }
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn test_interval_overlap(a: Interval, b: Interval) -> bool {
  return a.min < b.max && b.min < a.max;
}

///////////////////////////////////////////////////////////////////////////////

pub fn get_interval_overlap(a: Interval, b: Interval) -> f32 {
  // TODO switch to cmp in Rust
  if a.max > b.min {
    return a.max - b.min;
  }
  if b.max > a.min {
    return a.min - b.max;
  }
  return 0.0;
}

///////////////////////////////////////////////////////////////////////////////

pub fn test_point_v_circle(p: Vec2, c: Circle) -> bool {
  let delta_v: Vec2 = p - c.center;
  let x_sq = delta_v.x() * delta_v.x();
  let y_sq = delta_v.y() * delta_v.y();
  let r_sq = c.r * c.r;
  return (x_sq + y_sq) < r_sq;
}

///////////////////////////////////////////////////////////////////////////////

pub fn test_point_v_aabb(p: Vec2, aabb: Rect) -> bool {
  return p.x() > aabb.min_x && p.x() < aabb.max_x && p.y() > aabb.min_y && p.y() < aabb.max_y;
}

///////////////////////////////////////////////////////////////////////////////

pub fn test_aabb_v_aabb(a: Rect, b: Rect) -> bool {
  {
    let interval_a = Interval::new(a.min_y, a.max_y);
    let interval_b = Interval::new(b.min_y, b.max_y);
    if !test_interval_overlap(interval_a, interval_b) {
      return false;
    }
  }
  {
    let interval_a = Interval::new(a.min_x, a.max_x);
    let interval_b = Interval::new(b.min_x, b.max_x);
    if !test_interval_overlap(interval_a, interval_b) {
      return false;
    }
  }
  return true;
}

///////////////////////////////////////////////////////////////////////////////

pub fn test_overlap(a: Shape, b: Shape) -> bool {
  // sort the shapes to reduce the test s
  // if a > b
  // {
  //   let c = b;
  //   b = a;
  //   a = c;
  // }

  match (a, b) {
    (Shape::Point(a), Shape::Point(b)) => vec2_nearly(a, b),
    (Shape::Point(a), Shape::Circle(b)) | (Shape::Circle(b), Shape::Point(a)) => {
      test_point_v_circle(a, b)
    }
    (Shape::Point(a), Shape::Rect(b)) | (Shape::Rect(b), Shape::Point(a)) => {
      test_point_v_aabb(a, b)
    }
    (Shape::Circle(a), Shape::Circle(b)) => {
      // merge one circle into the other so we can do a simple point test
      let expanded_c = Circle {
        center: b.center,
        r: a.r + b.r,
      };
      test_point_v_circle(a.center, expanded_c)
    }

    (Shape::Circle(a), Shape::Rect(b)) | (Shape::Rect(b), Shape::Circle(a)) => {
      let corner = vec2(b.min_x, b.min_y);
      if test_point_v_circle(corner, a) {
        return true;
      }
      let corner = vec2(b.min_x, b.max_y);
      if test_point_v_circle(corner, a) {
        return true;
      }
      let corner = vec2(b.max_x, b.max_y);
      if test_point_v_circle(corner, a) {
        return true;
      }
      let corner = vec2(b.max_x, b.min_y);
      if test_point_v_circle(corner, a) {
        return true;
      }
      let aabb_y = Rect {
        min_x: b.min_x,
        max_x: b.max_x,
        min_y: b.min_y - a.r,
        max_y: b.max_y + a.r,
      };
      if test_point_v_aabb(a.center, aabb_y) {
        return true;
      }
      let aabb_x = Rect {
        min_x: b.min_x - a.r,
        max_x: b.max_x + a.r,
        min_y: b.min_y,
        max_y: b.max_y,
      };
      if test_point_v_aabb(a.center, aabb_x) {
        return true;
      }
      return false;
    }

    (Shape::Rect(a), Shape::Rect(b)) => test_aabb_v_aabb(a, b),
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn get_overlap_point_v_aabb(p: Vec2, aabb: Rect) -> OverlapResult {
  // start with the left edge
  let mut min_axis: Vec2 = left();
  let mut min_mag = p.x() - aabb.min_x;
  {
    let r_mag = aabb.max_x - p.x();
    if r_mag < min_mag {
      min_axis = right();
      min_mag = r_mag;
    }
  }
  {
    let u_mag = aabb.max_y - p.y();
    if u_mag < min_mag {
      min_axis = up();
      min_mag = u_mag;
    }
  }
  {
    let d_mag = p.y() - aabb.min_y;
    if d_mag < min_mag {
      min_axis = down();
      min_mag = d_mag;
    }
  }
  OverlapResult {
    normal: min_axis,
    distance: min_mag,
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn get_overlap_point_v_circle(p: Vec2, c: Circle) -> OverlapResult {
  let dist: Vec2 = p - c.center;
  let dist_mag = dist.length();
  let normal: Vec2 = Vec2::normalize(dist);
  OverlapResult {
    normal,
    distance: c.r - dist_mag,
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn get_overlap(a: Shape, b: Shape) -> OverlapResult {
  let mut result = OverlapResult {
    normal: Vec2::zero(),
    distance: f32::MAX,
  };

  // TODO not possible in Rust?
  // if a.type == SHAPE_NONE || b.type == SHAPE_NONE
  // {
  //   return result;
  // }

  match (a, b) {
    (Shape::Point(_), Shape::Point(_)) => result,
    (Shape::Point(a), Shape::Rect(b)) => get_overlap_point_v_aabb(a, b),
    (Shape::Point(a), Shape::Circle(b)) => get_overlap_point_v_circle(a, b),

    (Shape::Circle(a), Shape::Circle(b)) => {
      let p: Vec2 = a.center;
      let c: Circle = Circle {
        center: b.center,
        r: a.r + b.r,
      };
      return get_overlap_point_v_circle(p, c);
    }
    (Shape::Circle(a), Shape::Rect(b)) => {
      if a.center.x() > b.max_x && a.center.y() > b.max_y {
        let corner: Vec2 = vec2(b.max_x, b.max_y);
        let corner_c = Circle {
          center: corner,
          r: a.r,
        };
        return get_overlap_point_v_circle(a.center, corner_c);
      }
      if a.center.x() < b.min_x && a.center.y() > b.max_y {
        let corner = vec2(b.min_x, b.max_y);
        let corner_c = Circle {
          center: corner,
          r: a.r,
        };
        return get_overlap_point_v_circle(a.center, corner_c);
      }
      if a.center.x() > b.max_x && a.center.y() < b.min_y {
        let corner: Vec2 = vec2(b.max_x, b.min_y);
        let corner_c: Circle = Circle {
          center: corner,
          r: a.r,
        };
        return get_overlap_point_v_circle(a.center, corner_c);
      }
      if a.center.x() < b.min_x && a.center.y() < b.min_y {
        let corner: Vec2 = vec2(b.min_x, b.min_y);
        let corner_c: Circle = Circle {
          center: corner,
          r: a.r,
        };
        return get_overlap_point_v_circle(a.center, corner_c);
      }
      let aabb_expanded: Rect = Rect {
        min_x: b.min_x - a.r,
        max_x: b.max_x + a.r,
        min_y: b.min_y - a.r,
        max_y: b.max_y + a.r,
      };
      return get_overlap_point_v_aabb(a.center, aabb_expanded);
    }

    (Shape::Rect(a), Shape::Rect(b)) => {
      let mut min_distance = f32::MAX;
      let mut min_normal: Vec2 = Vec2::zero();
      {
        let distance = a.max_x - b.min_x;
        if distance < min_distance {
          min_distance = distance;
          min_normal = left();
        }
      }
      {
        let distance = b.max_x - a.min_x;
        if distance < min_distance {
          min_distance = distance;
          min_normal = right();
        }
      }
      {
        let distance = a.max_y - b.min_y;
        if distance < min_distance {
          min_distance = distance;
          min_normal = down();
        }
      }
      {
        let distance = b.max_y - a.min_y;
        if distance < min_distance {
          min_distance = distance;
          min_normal = up();
        }
      }
      result.distance = min_distance;
      result.normal = min_normal;
      return result;
    }

    _ => {
      // TODO is there a better way? This could result in an infinite loop for unhandled cases
      // and the compiler doesn't catch it
      result = get_overlap(b, a);
      result.normal = result.normal * -1.0;
      return result;
    }
  }
}

// ----------------------------------------------------------------------------
// sweep point v. edge
//
//   thanks, Wikipedia.

pub fn sweep_point_v_edge(result: &mut SweepResult, step: Vec2, p: Vec2, edge: LineSegment) {
  // TODO when doing several of these in series for the same p and step,
  // this calc can be redundant. Perhaps I could prime it as an extra param.
  let p_stepped: Vec2 = (p + step);

  // TODO points in right order?
  let x1 = p.x();
  let y1 = p.y();
  let x2 = p_stepped.x();
  let y2 = p_stepped.y();

  let x3 = edge.a.x();
  let y3 = edge.a.y();
  let x4 = edge.b.x();
  let y4 = edge.b.y();

  // this gets the "time" of intersection along one line with the other
  // t < 0 means the intersection is before the first point and
  // t > 0 means the intersection is after the second (stepped) point
  let t_denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
  if t_denominator == 0.0 {
    // this can happen when the lines are parallel
    return;
  }
  let t_numerator = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
  let t = t_numerator / t_denominator;
  if t <= 0.0 || t >= 1.0 {
    // TODO epsilon?
    return;
  }

  // t only gets us the time of the intersection with respect to the
  // line that the edge falls on. We need to also make sure the
  // intersection falls between the
  let u_numerator = (x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3);
  let u_denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
  let u = -u_numerator / u_denominator;

  if u <= 0.0 || u >= 1.0 {
    // TODO epsilon?
    return;
  }

  if t >= 0.0 && t <= 1.0 {
    if t < result.t {
      result.t = t;
      result.normal = edge.normal;
    }
  }
}

// ----------------------------------------------------------------------------
// sweep point v. aabb

pub fn sweep_point_v_aabb(result: &mut SweepResult, step: Vec2, p: Vec2, aabb: Rect) {
  if step.x() > 0.0 {
    let edge = aabb.left_edge();
    sweep_point_v_edge(result, step, p, edge);
  } else {
    let edge = aabb.right_edge();
    sweep_point_v_edge(result, step, p, edge);
  }
  if step.y() > 0.0 {
    let edge = aabb.bottom_edge();
    sweep_point_v_edge(result, step, p, edge);
  } else {
    let edge = aabb.top_edge();
    sweep_point_v_edge(result, step, p, edge);
  }
}

///////////////////////////////////////////////////////////////////////////////

pub fn sweep_point_v_circle(result: &mut SweepResult, step: Vec2, p: Vec2, circle: Circle) {
  // shamelessly borrowed from https://stackoverflow.com/a/1084899
  let e: Vec2 = p; // E is the starting point of the ray,
  let l: Vec2 = p + step; // L is the end point of the ray,

  let c: Vec2 = circle.center; // C is the center of sphere you're testing against
  let r = circle.r;

  let d: Vec2 = l - e; // d = L - E (Direction vector of ray, from start to end)
  let f: Vec2 = e - c; // f = E - C (Vector from center sphere to ray start)

  let a = Vec2::dot(d, d);
  let b = 2.0 * Vec2::dot(f, d);
  let c = Vec2::dot(f, f) - r * r;

  let discriminant = b * b - 4.0 * a * c;
  if discriminant < NEAR_ZERO {
    // no intersection
    return;
  }

  // ray didn't totally miss sphere,
  // so there is a solution to
  // the equation.
  let discriminant = f32::sqrt(discriminant);

  // either solution may be on or off the ray so need to test both
  // t1 is always the smaller value, because BOTH discriminant and
  // a are nonnegative.
  let t1 = (-b - discriminant) / (2.0 * a);
  let t2 = (-b + discriminant) / (2.0 * a);

  // 6x cases:
  //  ---(-----)-->           ---(-->  )            (   --)->
  // Impale(t1 hit,t2 hit), Poke(t1 hit,t2>1), ExitWound(t1<0, t2 hit),
  //
  //  -> (     )                 (     ) ->         ( --> )
  // FallShort (t1>1,t2>1), Past (t1<0,t2<0), CompletelyInside(t1<0, t2>1)

  // FallShort, Past
  // in these cases, there is no collision to handle, so just move along
  if (t1 > 1.0) || (t1 < 0.0) {
    return;
  }
  // We now know that somewhere along the axis of movement, the two bodies
  // overlap.

  let t = if t1 < t2 { t1 } else { t2 };

  // TODO in the AABB check, I could early out here if t is not less than
  // a previous result
  if t > result.t {
    return;
  }

  // in all other scenarios, we're chosing to project the offending entity
  // back out of collision along its vector of motion (result.step). Along
  // the ray, t1 is always the "first" hit (which may mean moving backward)
  result.t = t;
  let collision_pos: Vec2 = (p + (step * t));
  result.normal = Vec2::normalize((collision_pos - circle.center));
  return;
}

// ----------------------------------------------------------------------------
// A continuous collision detector.
//
//  Takes in a step vector and two shapes. The first shape is assumed to be
//  the one in motion. The result object will include a .t value which will
//  be 1.0 if no collision is found along the step, and will be a value
//  between 0 and 1 when a collision is found, indicating the per

pub fn sweep(step: Vec2, a: Shape, b: Shape) -> SweepResult {
  let mut result = SweepResult {
    t: 1.0,
    normal: Vec2::zero(),
  };

  match (a, b) {
    (Shape::Point(_), Shape::Point(_)) => {
      // point shapes can't really collide with each other; they're too precise
      return result;
    }
    (Shape::Point(a), Shape::Rect(b)) => {
      sweep_point_v_aabb(&mut result, step, a, b);
      return result;
    }
    (Shape::Point(a), Shape::Circle(b)) => {
      sweep_point_v_circle(&mut result, step, a, b);
      return result;
    }

    (Shape::Circle(a), Shape::Circle(b)) => {
      let p: Vec2 = a.center;
      let c: Circle = Circle {
        center: b.center,
        r: a.r + b.r,
      };
      sweep_point_v_circle(&mut result, step, p, c);
      return result;
    }
    (Shape::Circle(a), Shape::Rect(b)) => {
      // TODO I might be able to exclude the "back" corner
      let corners: [Vec2; 4] = [
        vec2(b.max_x, b.max_y),
        vec2(b.min_x, b.max_y),
        vec2(b.max_x, b.min_y),
        vec2(b.min_x, b.min_y),
      ];
      for i in 0..4 {
        let corner: Vec2 = corners[i];
        let corner_c: Circle = Circle {
          center: corner,
          r: a.r,
        };
        sweep_point_v_circle(&mut result, step, a.center, corner_c);
      }

      // expand aabb on each axis
      {
        let aabb_expanded_x = Rect {
          min_x: b.min_x - a.r,
          max_x: b.max_x + a.r,
          min_y: b.min_y,
          max_y: b.max_y,
        };
        sweep_point_v_aabb(&mut result, step, a.center, aabb_expanded_x);
      }
      {
        let aabb_expanded_y = Rect {
          min_x: b.min_x,
          max_x: b.max_x,
          min_y: b.min_y - a.r,
          max_y: b.max_y + a.r,
        };
        sweep_point_v_aabb(&mut result, step, a.center, aabb_expanded_y);
      }

      return result;
    }

    (Shape::Rect(a), Shape::Rect(b)) => {
      // thanks to
      // https://www.gamedev.net/articles/programming/general-and-gameplay-programming/swept-aabb-collision-detection-and-response-r3084/

      // check broad first
      {
        // create a bounding box around the swept volume to do
        // a broad phase check first
        let broad_a = Rect {
          min_x: if step.x() > 0.0 {
            a.min_x
          } else {
            a.min_x + step.x()
          },
          max_x: if step.x() < 0.0 {
            a.max_x
          } else {
            a.max_x + step.x()
          },
          min_y: if step.y() > 0.0 {
            a.min_y
          } else {
            a.min_y + step.y()
          },
          max_y: if step.y() < 0.0 {
            a.max_y
          } else {
            a.max_y + step.y()
          },
        };
        if !test_aabb_v_aabb(broad_a, b) {
          return result;
        }
      }

      let x_inv_entry: f32;
      let y_inv_entry: f32;
      let x_inv_exit: f32;
      let y_inv_exit: f32;

      // find the distance between the objects on the near and far sides for
      // both x and y
      if step.x() > 0.0 {
        x_inv_entry = b.min_x - (a.max_x);
        x_inv_exit = (b.max_x) - a.min_x;
      } else {
        x_inv_entry = (b.max_x) - a.min_x;
        x_inv_exit = b.min_x - (a.max_x);
      }

      if step.y() > 0.0 {
        y_inv_entry = b.min_y - (a.max_y);
        y_inv_exit = (b.max_y) - a.min_y;
      } else {
        y_inv_entry = (b.max_y) - a.min_y;
        y_inv_exit = b.min_y - (a.max_y);
      }

      // find time of collision and time of leaving for each axis (if statement
      // is to prevent divide by zero)
      let x_entry: f32;
      let y_entry: f32;
      let x_exit: f32;
      let y_exit: f32;

      if step.x() == 0.0 {
        x_entry = f32::MIN;
        x_exit = f32::MAX;
      } else {
        x_entry = x_inv_entry / step.x();
        x_exit = x_inv_exit / step.x();
      }

      if step.y() == 0.0 {
        y_entry = f32::MIN;
        y_exit = f32::MAX;
      } else {
        y_entry = y_inv_entry / step.y();
        y_exit = y_inv_exit / step.y();
      }

      // find the earliest/latest times of collision
      let entry_time = max(x_entry, y_entry);
      let exit_time = min(x_exit, y_exit);

      if entry_time > result.t {
        return result; // an earlier collistion is already found
      }

      // if there was no collision
      if entry_time > exit_time
        || (x_entry < 0.0 && y_entry < 0.0)
        || x_entry > 1.0
        || y_entry > 1.0
      {
        return result;
      }
      result.t = entry_time;

      // calculate normal of collided surface
      if x_entry > y_entry {
        if x_inv_entry < 0.0 {
          result.normal = vec2(1.0, 0.0);
        } else {
          result.normal = vec2(-1.0, 0.0);
        }
      } else {
        if y_inv_entry < 0.0 {
          result.normal = vec2(0.0, 1.0);
        } else {
          result.normal = vec2(0.0, -1.0);
        }
      }

      // return the time of collision
      return result;
    }
    (_, _) => {
      // reversed test cases handled here to reduce the above matches;
      // TODO compiler won't catch unhandled cases here and missing a case will
      // result in an infinite loop. Is there a better way?
      let mut result = sweep(step * -1.0, b, a);
      result.normal = result.normal * -1.0;
      return result;
    }
  }
}

pub fn rand_in_shape(shape: Shape) -> Vec2 {
  match shape {
    Shape::Point(p) => p,
    Shape::Rect(r) => {
      let x = lerpf(r.min_x, r.max_x, rand::random());
      let y = lerpf(r.min_y, r.max_y, rand::random());
      return vec2(x, y);
    }
    Shape::Circle(c) => {
      let random_arc: f32 = rand::random();
      let a: f32 = random_arc * TAU;
      let r = c.r * f32::sqrt(rand::random());

      // If you need it in Cartesian coordinates
      let x = r * f32::cos(a);
      let y = r * f32::sin(a);
      return c.center + vec2(x, y);
    }
  }
}
