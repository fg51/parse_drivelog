pub mod record;

use record::Record;
// use record::{CurrentMotor, Displacement, Emag, Temparature};

pub struct AverageBuilder {
    length: usize,
    record: Record,
}

impl AverageBuilder {
    pub fn new(length: usize) -> Self {
        Self {
            length: length,
            record: Record::new(),
        }
    }

    pub fn build(&self) -> Record {
        self.record.clone()
    }

    pub fn setup(&mut self, xs: Vec<Record>) {
        self.record.speed = self.average(xs.iter().map(|x| x.speed));
        self.record.rated_speed = self.average(xs.iter().map(|x| x.rated_speed));

        self.record.current_motor.u = self.average(xs.iter().map(|x| x.current_motor.u));
        self.record.current_motor.v = self.average(xs.iter().map(|x| x.current_motor.v));
        self.record.current_motor.w = self.average(xs.iter().map(|x| x.current_motor.w));
        self.record.current_motor.ave =
            self.average(xs.iter().map(|x| x.current_motor.ave));

        self.record.temparature.motor =
            self.average(xs.iter().map(|x| x.temparature.motor));
        self.record.temparature.pk = self.average(xs.iter().map(|x| x.temparature.pk));
        self.record.temparature.main =
            self.average(xs.iter().map(|x| x.temparature.main));
        self.record.temparature.case =
            self.average(xs.iter().map(|x| x.temparature.case));

        self.record.displacement.x1 = self.average(xs.iter().map(|x| x.displacement.x1));
        self.record.displacement.y1 = self.average(xs.iter().map(|x| x.displacement.y1));
        self.record.displacement.x2 = self.average(xs.iter().map(|x| x.displacement.x2));
        self.record.displacement.y2 = self.average(xs.iter().map(|x| x.displacement.y2));
        self.record.displacement.z = self.average(xs.iter().map(|x| x.displacement.z));
        self.record.displacement.r1 = self.average(xs.iter().map(|x| x.displacement.r1));
        self.record.displacement.r2 = self.average(xs.iter().map(|x| x.displacement.r2));

        self.record.emag.x1_p = self.average(xs.iter().map(|x| x.emag.x1_p));
        self.record.emag.x1_n = self.average(xs.iter().map(|x| x.emag.x1_n));
        self.record.emag.y1_p = self.average(xs.iter().map(|x| x.emag.y1_p));
        self.record.emag.y1_n = self.average(xs.iter().map(|x| x.emag.y1_n));
        self.record.emag.x2_p = self.average(xs.iter().map(|x| x.emag.x2_p));
        self.record.emag.x2_n = self.average(xs.iter().map(|x| x.emag.x2_n));
        self.record.emag.y2_p = self.average(xs.iter().map(|x| x.emag.y2_p));
        self.record.emag.y2_n = self.average(xs.iter().map(|x| x.emag.y2_n));
        self.record.emag.z1_p = self.average(xs.iter().map(|x| x.emag.z1_p));
        self.record.emag.z1_n = self.average(xs.iter().map(|x| x.emag.z1_n));

        self.record.unb_upper = self.average(xs.iter().map(|x| x.unb_upper));
        self.record.unb_lower = self.average(xs.iter().map(|x| x.unb_lower));
    }

    pub fn average(&self, xs: impl Iterator<Item = f64>) -> f64 {
        average(xs, self.length as f64)
    }
}

fn average(xs: impl Iterator<Item = f64>, length: f64) -> f64 {
    xs.fold(0., |sum, x| sum + x) / length
}
