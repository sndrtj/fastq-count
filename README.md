# fastq-count

Simple fastq read and base counter for paired data. 

## Usage

Running the counter is as simple as:

`fastq-count r1.fq r2.fq`

Fastq files may be gzip compressed. When they are, their filenames must end in
`.gz`. 

fastq-count returns a simple json, with schema:

```json
{
  "reads": {"type": "integer"},
  "bases": {"type": "integer"}
}
```

## License

MIT
