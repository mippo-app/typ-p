# typ-p

typ is difine primitive type proto files.


- typ-p provide pre-difined types for primitive use:

| Type | Status |
|:----:|:------:|
| bool | yes |
| string | yes |
| bytes | yes |
| uuid  | yes |
| number | yes |
| datetime | yes |
| date | yes |
| date_range | yes |
| array_value | yes |



## Examples

```proto
message value {
  oneof value_of {
    bool bool_value = 1;
    string string_value = 2;
    //byte
    bytes bytes_value = 3;
    //uuid
    uuid uuid_value = 4;
    
    number number_value = 5;

    date_time date_time_value = 8;

    date date_value = 9;
    date_range date_range_value = 10;
    
    array_value array_values = 11;
  }
}
```


