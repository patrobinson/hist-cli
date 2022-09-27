extern crate csv;
use csv::StringRecord;
use std::io;
use std::collections::BTreeMap;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use lexical_sort::StringSort;


#[allow(non_snake_case)]
#[derive(Debug, StructOpt)]
#[structopt(name = "hist", about = "Plots histogram of input", rename_all="verbatim")]
struct Opt
{
    #[structopt(parse(from_os_str))]
    /// optional file with on entry per line [default: STDIN]
    input: Option<PathBuf>,

    // r"" makes it prinable as escaped in default
    #[structopt(short, long, default_value = r"\t")]
    /// column delimiter
    delimiter: String,

    #[structopt(long, short, default_value = "1")]
    /// key (column) selector
    key: usize,
}

fn main()
{
    let opt = Opt::from_args();

    let input: Box<dyn std::io::Read + 'static> =
        if let Some(path) = &opt.input
        {
            Box::new(File::open(&path).unwrap())
        }
        else
        {
            Box::new(io::stdin())
        };

    // accept escaped delimiters
    // could be expanded to aliases e.g. "TAB"
    let delimiter = match opt.delimiter.as_str()
    {
        r"\t" => b'\t', // structopt needs r"" to show default as escaped, also for sepcifiying as escaped in CLI
         _ => *opt.delimiter.as_bytes().first().expect("Not a valid delimiter")
    };

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(input);

    let mut key_counts = BTreeMap::new();
    let mut record = StringRecord::new();
    while reader.read_record(&mut record).unwrap()
    {
       let s = record.get(opt.key - 1).unwrap().to_string(); 
       key_counts.entry(s).and_modify(|e| *e += 1 ).or_insert(1);
    }

    text_plot(&key_counts);

}

fn text_plot(histogram : &BTreeMap<String, usize>)
{
    let mut keys = Vec::from_iter(histogram.keys().map(|k| k.as_str()));

    keys.string_sort_unstable(lexical_sort::natural_lexical_cmp);
    for key in keys {
        let count = histogram[key];
        let bars = "#".repeat(count as usize);
        println!("{}	{}	{}", key, count, bars)
    }
}
