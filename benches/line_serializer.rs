#![feature(test)]

extern crate test;
extern crate influent;

#[cfg(test)]
mod tests {
    use influent::serializer::line::{LineSerializer};
    use influent::serializer::Serializer;
    use influent::measurement::{Measurement, Value};
    use test::Bencher;

    #[bench]
    fn test_line_serializer(b: &mut Bencher) {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));
        measurement.add_field("i", Value::Integer(10));
        measurement.add_field("f", Value::Float(10f64));
        measurement.add_field("b", Value::Boolean(false));

        measurement.add_tag("tag", "value");
        
        measurement.add_field("one, two", Value::String("three"));
        measurement.add_tag("one ,two", "three, four");


        measurement.set_timestamp(10);

    	b.iter(|| serializer.serialize(&measurement));
    }

    #[bench]
    fn test_line_serializer_single_string(b: &mut Bencher) {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));

        b.iter(|| serializer.serialize(&measurement));
    }

    #[bench]
    fn test_line_serializer_double_string(b: &mut Bencher) {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));

        measurement.add_field("s", Value::String("string"));

        b.iter(|| serializer.serialize(&measurement));
    }

    #[bench]
    fn test_line_serializer_single_string_with_timestamp(b: &mut Bencher) {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));

        measurement.set_timestamp(1434055562000000000);

        b.iter(|| serializer.serialize(&measurement));
    }

    #[bench]
    fn test_line_serializer_double_string_with_timestamp(b: &mut Bencher) {
        let serializer = LineSerializer::new();
        let mut measurement = Measurement::new("key");

        measurement.add_field("s", Value::String("string"));
        
        measurement.add_field("s", Value::String("string"));

        measurement.set_timestamp(1434055562000000000);

        b.iter(|| serializer.serialize(&measurement));
    }
}




