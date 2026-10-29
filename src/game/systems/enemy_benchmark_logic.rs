use bevy::prelude::*;

use crate::{
    game::components::enemy::{Enemy, enemy_state::WorkloadCursor},
    random_table,
};

pub fn enemy_benchmark(mut enemy_query: Query<&mut WorkloadCursor, With<Enemy>>) {
    let table = random_table::table();
    enemy_query.iter_mut().for_each(|mut workload_cursor| {
        let mut cursor = workload_cursor.0;
        let mut sum = 0.0_f64;
        for _ in 0..7_000 {
            let i = table.workload_10k(cursor);
            cursor = cursor.wrapping_add(1);
            sum += (i as f64 * std::f64::consts::PI).sqrt() * (i as f64 * 0.01).cos();
        }
        workload_cursor.0 = cursor;

        if sum.is_sign_negative() {
            println!("weird");
        }
    });
}
