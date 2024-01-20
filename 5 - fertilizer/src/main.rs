use std::{fs::File, io::{BufReader, BufRead}, cmp};

struct SingleMapper {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl SingleMapper {
    pub fn map_src_dest(&self, number: u64) -> u64 {
        if self.source_range_start <= number && number <= self.source_range_start + self.range_length {
            let dist = number - self.source_range_start;
            return self.destination_range_start + dist;
        }

        number
    }

    pub fn map_intersect_source(&self, m: MapperRange) -> (Option<MapperRange>, Vec<MapperRange>) {
        let mut ret_vec: Vec<MapperRange> = vec![];

        // no intersection
        if m.start < self.source_range_start && m.start + m.length < self.source_range_start ||
            m.start > self.source_range_start + self.range_length {
                ret_vec.push(m);
                return (None, ret_vec);
        }
        
        let mut intersect_start: u64 = cmp::max(m.start, self.source_range_start);
        let intersect_length: u64 = cmp::min(m.start + m.length, self.source_range_start + self.range_length) - intersect_start;

        let mut used_lenght = intersect_length;

        // non-intersecting below the lower bound
        if m.start < intersect_start {
            let lower_intersect_length = intersect_start - m.start;
            used_lenght += lower_intersect_length;
            ret_vec.push(MapperRange { start: m.start, length: lower_intersect_length})
        }

        // non-intersecting above the higher bound
        if m.start + m.length > intersect_start + intersect_length {
            let non_intersect_length: u64 = m.length - used_lenght;
            ret_vec.push(MapperRange { start: intersect_start + intersect_length + 1, length: non_intersect_length});
        }

        let dist = intersect_start - self.source_range_start;
        intersect_start = self.destination_range_start + dist;

        (Some(MapperRange{ start: intersect_start, length: intersect_length }), ret_vec)
    }
}

#[derive(Debug, Copy, Clone)]
struct MapperRange {
    start: u64,
    length: u64,
}

struct Mapper {
    mappers: Vec<SingleMapper>,
}

impl Mapper {
    pub fn map_src_dest(&self, number: u64) -> u64 {
        for mapper in &self.mappers {
            let res = mapper.map_src_dest(number);
            if res != number {
                return res;
            }
        }

        number
    }

    pub fn map_src_dest_interval(&self, r: MapperRange) -> Vec<MapperRange> {
        let mut mapped: Vec<MapperRange> = vec![];
        let mut non_mapped: Vec<MapperRange> = vec![];

        non_mapped.push(r);

        for mapper in &self.mappers {
            let mut temp_non_mapped: Vec<MapperRange> = vec![];
            for m in &non_mapped {
                let res = mapper.map_intersect_source(*m);
                let res_mapped: Option<MapperRange> = res.0;
                let res_non_mapped: Vec<MapperRange> = res.1;

                if res_mapped.is_some() {
                    mapped.push(res_mapped.unwrap());
                }

                for r in res_non_mapped {
                    temp_non_mapped.push(r);
                }
            }
            non_mapped.clear();
            for r in temp_non_mapped {
                non_mapped.push(r);
            }
        }
        let mut ret: Vec<MapperRange> = vec![];
        for r in mapped {
            ret.push(r);
        }
        for r in non_mapped {
            ret.push(r);
        }

        ret
    }
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader = BufReader::new(file);

    let mut seed_to_soil: Mapper = Mapper { mappers: vec![] };
    let mut soil_to_fertilizer: Mapper = Mapper{ mappers: vec![] };
    let mut fertiliezer_to_water: Mapper = Mapper{ mappers: vec![] };
    let mut water_to_light: Mapper = Mapper{ mappers: vec![] };
    let mut light_to_temperature: Mapper = Mapper{ mappers: vec![] };
    let mut temperature_to_humidity: Mapper = Mapper{ mappers: vec![] };
    let mut humidity_to_location: Mapper = Mapper{ mappers: vec![] };

    let mut current_map: &mut Mapper = &mut seed_to_soil;

    let mut seeds: Vec<u64> = vec![];

    let mut line: String = String::new();
    let mut linelen = reader.read_line(&mut line)?;

    while linelen != 0 {
        let trim_line = line.trim_end();
        if trim_line.starts_with("seeds:") {
            let (_, seeds_string) = trim_line.trim_end().split_at(7);
            let seed_values: Vec<&str> = seeds_string.split(' ').collect();

            for seed_value in seed_values {
                let val = seed_value.parse::<u64>().unwrap();
                seeds.push(val);
            }

        } else if trim_line.starts_with("seed-to-soil map:") {
            current_map = &mut seed_to_soil;
        } else if line.starts_with("soil-to-fertilizer map:") {
            current_map = &mut soil_to_fertilizer;
        } else if trim_line.starts_with("fertilizer-to-water map:") {
            current_map = &mut fertiliezer_to_water;
        } else if trim_line.starts_with("water-to-light map:") {
            current_map = &mut water_to_light;
        } else if trim_line.starts_with("light-to-temperature map:") {
            current_map = &mut light_to_temperature;
        } else if trim_line.starts_with("temperature-to-humidity map:") {
            current_map = &mut temperature_to_humidity;
        } else if trim_line.starts_with("humidity-to-location map:") {
            current_map = &mut humidity_to_location;
        } else if trim_line.len() != 0 {
            let str_values: Vec<&str> = line.trim_end().split(' ').collect();
            let destination_range_start = str_values[0].parse::<u64>().unwrap();
            let source_range_start = str_values[1].parse::<u64>().unwrap();
            let range_length = str_values[2].parse::<u64>().unwrap();
            current_map.mappers.push(SingleMapper { destination_range_start, source_range_start, range_length });
        }

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    let mut result = u64::MAX;

    for seed in seeds.iter() {
        let soil = seed_to_soil.map_src_dest(*seed);
        let fertilizer = soil_to_fertilizer.map_src_dest(soil);
        let water = fertiliezer_to_water.map_src_dest(fertilizer);
        let light = water_to_light.map_src_dest(water);
        let temperature = light_to_temperature.map_src_dest(light);
        let humidity = temperature_to_humidity.map_src_dest(temperature);
        let location = humidity_to_location.map_src_dest(humidity);

        if result > location {
            result = location;
        }
    }

    println!("{}", result);

    // part 2
    let mappers: Vec<Mapper> = vec![seed_to_soil, soil_to_fertilizer, fertiliezer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location];

    let mut start: u64 = 0;
    let mut stepper = 0;

    let mut intervals: Vec<MapperRange> = vec![];
    let mut result2 = u64::MAX;

    for seed in seeds.iter() {

        if stepper % 2 == 0 {
            start = *seed;
        } else {
            let range = *seed;
            intervals.clear();
            intervals.push(MapperRange{ start, length: range });
            for mapper in &mappers {
                let mut res_intervals: Vec<MapperRange> = vec![];
                for interval in intervals {
                    let temp_res = mapper.map_src_dest_interval(MapperRange { start: interval.start, length: interval.length });
                    for r in temp_res {
                        res_intervals.push(r);
                    }
                }
                intervals = res_intervals;
            }
            for i in &intervals {
                if i.start < result2 {
                    result2 = i.start;
                }
            }
        }
        stepper += 1;
    }
    println!("{}", result2);

    Ok(())
}
