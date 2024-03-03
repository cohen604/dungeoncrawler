use crate::prelude::*;

const NUM_ROOMS: usize = 20;
const ROOM_MIN_SIZE: i32 = 2;
const ROOM_MAX_SIZE: i32 = 10;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::with_capacity(NUM_ROOMS),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - ROOM_MAX_SIZE),
                rng.range(1, SCREEN_HEIGHT - ROOM_MAX_SIZE),
                rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE),
                rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE),
            );
            let mut overlap = false;

            for r in self.rooms.iter() {
                if room.intersect(r) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if is_point_on_map(p) {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1];
            let start = room.center();
            let end = prev.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(start.x, end.x, start.y);
                self.apply_vertical_tunnel(start.y, end.y, end.x);
            } else {
                self.apply_vertical_tunnel(start.y, end.y, start.x);
                self.apply_horizontal_tunnel(start.x, end.x, end.y);
            }
        }
    }
}