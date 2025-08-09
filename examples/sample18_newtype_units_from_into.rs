#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[allow(dead_code)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

// 力の単位
struct PoundForceSeconds(pub f64);
impl From<PoundForceSeconds> for NewtonSeconds {
    fn from(pfs: PoundForceSeconds) -> Self {
        NewtonSeconds(pfs.0 * 4.44822) // 1 lbf = 4.44822 N
    }
}

// スラスタに点火。生み出されるインパルスを返す
fn thruster_impulse(direction: Direction) -> PoundForceSeconds {
    match direction {
        Direction::Forward => PoundForceSeconds(10.0),
        Direction::Backward => PoundForceSeconds(-10.0),
        Direction::Left => PoundForceSeconds(5.0),
        Direction::Right => PoundForceSeconds(-5.0),
    }
}

/// 力の単位
struct NewtonSeconds(pub f64);
// インパルスに応じて軌跡モデルを更新
fn update_trajectory(force: NewtonSeconds) {
    // ここでは何もしない
    println!("Updating trajectory with force: {} N·s", force.0);
}

fn main() {
    // スラスタを点火してインパルスを得る
    let impulse = thruster_impulse(Direction::Forward);

    // ✖ PoundForceSecoundsはNewtonSecondsとは異なるため、意図しない代入を拒否できる
    // update_trajectory(impulse);

    // 軌跡モデルを更新
    update_trajectory(impulse.into()); // Fromトレイトを使って変換できる
}

