use serde::Deserialize;

#[derive(Debug)]
pub struct Record {
    pub date_time: String,
    pub passed_sec: f64,  // [sec]
    pub speed: f64,       // [rps]
    pub rated_speed: f64, // [rps]
    pub speed_ratio: f64, //[%]

    pub current_motor: CurrentMotor,
    pub temparature: Temparature,
    pub displacement: Displacement,
    pub emag: Emag,

    pub unb_upper: f64,      // UNB [%]
    pub unb_lower: f64,      // UNB [%]
    pub drive_time: f64,     //[hrs]
    pub cumulated_hour: f64, // [hrs]
}

#[derive(Debug)]
pub struct CurrentMotor {
    pub u: f64,   // [A]
    pub v: f64,   // [A]
    pub w: f64,   // [A]
    pub ave: f64, // [A]
}

#[derive(Debug)]
pub struct Temparature {
    pub motor: f64, // [degC]
    pub pk: f64,    // [degC]
    pub main: f64,  // [degC]
    pub case: f64,  // [degC]
}

#[derive(Debug)]
pub struct Displacement {
    pub x1: f64, // [um]
    pub y1: f64, // [um]
    pub x2: f64, // [um]
    pub y2: f64, // [um]
    pub z: f64,  //  [um]
    pub r1: f64, //  [um]
    pub r2: f64, // [um]
}

#[derive(Debug)]
pub struct Emag {
    pub x1_p: f64, // [A]
    pub x1_n: f64, // [A]
    pub y1_p: f64, // [A]
    pub y1_n: f64, // [A]
    pub x2_p: f64, // [A]
    pub x2_n: f64, // [A]
    pub y2_p: f64, // [A]
    pub y2_n: f64, // [A]
    pub z1_p: f64, // [A]
    pub z1_n: f64, // [A]
}

impl Record {
    pub fn new(r: RawRecord) -> Self {
        Self {
            date_time: r.date_time,
            passed_sec: r.passed_sec,
            speed: r.speed,
            rated_speed: r.rated_speed,
            speed_ratio: r.speed_ratio,

            current_motor: CurrentMotor {
                u: r.current_motor0,
                v: r.current_motor1,
                w: r.current_motor2,
                ave: r.current_motor_ave,
            },
            temparature: Temparature {
                motor: r.temparature_motor,
                pk: r.temparature_pk,
                main: r.temparature_main,
                case: r.temparature_case,
            },
            displacement: Displacement {
                x1: r.disp_x1,
                y1: r.disp_y1,
                x2: r.disp_x2,
                y2: r.disp_y2,
                z: r.disp_z,
                r1: r.disp_r1,
                r2: r.disp_r2,
            },
            emag: Emag {
                x1_p: r.emag_x1_p,
                x1_n: r.emag_x1_n,
                y1_p: r.emag_y1_p,
                y1_n: r.emag_y1_n,
                x2_p: r.emag_x2_p,
                x2_n: r.emag_x2_n,
                y2_p: r.emag_y2_p,
                y2_n: r.emag_y2_n,
                z1_p: r.emag_z1_p,
                z1_n: r.emag_z1_n,
            },

            unb_upper: r.unb_upper,
            unb_lower: r.unb_lower,
            drive_time: r.drive_time,
            cumulated_hour: r.cumulated_hour,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawRecord {
    pub date_time: String,
    pub passed_sec: f64,        // [sec]
    pub speed: f64,             // [rps]
    pub rated_speed: f64,       // [rps]
    pub speed_ratio: f64,       //[%]
    pub current_motor0: f64,    // [A]
    pub current_motor1: f64,    // [A]
    pub current_motor2: f64,    // [A]
    pub current_motor_ave: f64, // [A]
    pub temparature_motor: f64, // [degC]
    pub temparature_pk: f64,    // [degC]
    pub temparature_main: f64,  // [degC]
    pub temparature_case: f64,  // [degC]
    pub disp_x1: f64,           // [um]
    pub disp_y1: f64,           // [um]
    pub disp_x2: f64,           // [um]
    pub disp_y2: f64,           // [um]
    pub disp_z: f64,            //  [um]
    pub disp_r1: f64,           //  [um]
    pub disp_r2: f64,           // [um]
    pub emag_x1_p: f64,         // [A]
    pub emag_x1_n: f64,         // [A]
    pub emag_y1_p: f64,         // [A]
    pub emag_y1_n: f64,         // [A]
    pub emag_x2_p: f64,         // [A]
    pub emag_x2_n: f64,         // [A]
    pub emag_y2_p: f64,         // [A]
    pub emag_y2_n: f64,         // [A]
    pub emag_z1_p: f64,         // [A]
    pub emag_z1_n: f64,         // [A]
    pub unb_upper: f64,         // UNB [%]
    pub unb_lower: f64,         // UNB [%]
    pub drive_time: f64,        //[hrs]
    pub cumulated_hour: f64,    // [hrs]
}

// fn to_average(xs: Vec<Record>) -> Record {
//     // let speed_ave = data.iter().map(|x| x.speed).fold(0., |sum, x| sum + x) / data.len() as f64;
//     let length = xs.len() as f64;
//
//     Self {
//         date_time: "".to_string(),
//         passed_sec: 0.0,
//         speed: fold_ave(length, xs.iter().map(|x| x.rated_speed)),
//         rated_speed: xs.iter().map(|x| x.rated_speed).fold(0., |sum, x| sum + x) / length,
//         speed_ratio: r.speed_ratio,
//
//         current_motor: CurrentMotor {
//             u: r.current_motor0,
//             v: r.current_motor1,
//             w: r.current_motor2,
//             ave: r.current_motor_ave,
//         },
//         temparature: Temparature {
//             motor: r.temparature_motor,
//             pk: r.temparature_pk,
//             main: r.temparature_main,
//             case: r.temparature_case,
//         },
//         displacement: Displacement {
//             x1: r.disp_x1,
//             y1: r.disp_y1,
//             x2: r.disp_x2,
//             y2: r.disp_y2,
//             z: r.disp_z,
//             r1: r.disp_r1,
//             r2: r.disp_r2,
//         },
//         emag: Emag {
//             x1_p: r.emag_x1_p,
//             x1_n: r.emag_x1_n,
//             y1_p: r.emag_y1_p,
//             y1_n: r.emag_y1_n,
//             x2_p: r.emag_x2_p,
//             x2_n: r.emag_x2_n,
//             y2_p: r.emag_y2_p,
//             y2_n: r.emag_y2_n,
//             z1_p: r.emag_z1_p,
//             z1_n: r.emag_z1_n,
//         },
//
//         unb_upper: r.unb_upper,
//         unb_lower: r.unb_lower,
//         drive_time: r.drive_time,
//         cumulated_hour: r.cumulated_hour,
//     }
// }
