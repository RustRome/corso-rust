### Error Handling

The [Error
Handling](https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html)
and [Generics](https://doc.rust-lang.org/book/second-edition/ch10-01-syntax.html) sections are
relevant.

- ["option1.rs"](https://play.rust-lang.org/?code=%2F%2F+option1.rs%0A%2F%2F+This+example+panics+because+the+second+time+it+calls+%60pop%60%2C+the+%60vec%60%0A%2F%2F+is+empty%2C+so+%60pop%60+returns+%60None%60%2C+and+%60unwrap%60+panics+if+it%27s+called%0A%2F%2F+on+%60None%60.+Handle+this+in+a+more+graceful+way+than+calling+%60unwrap%60%21%0A%2F%2F+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+mut+list+%3D+vec%21%5B3%5D%3B%0A%0A++++let+last+%3D+list.pop%28%29.unwrap%28%29%3B%0A++++println%21%28%22The+last+item+in+the+list+is+%7B%3A%3F%7D%22%2C+last%29%3B%0A%0A++++let+second_to_last+%3D+list.pop%28%29.unwrap%28%29%3B%0A++++println%21%28%22The+second-to-last+item+in+the+list+is+%7B%3A%3F%7D%22%2C+second_to_last%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Try+using+a+%60match%60+statement+where+the+arms+are+%60Some%28thing%29%60+and+%60None%60.%0A%2F%2F+Or+set+a+default+value+to+print+out+if+you+get+%60None%60+by+using+the%0A%2F%2F+function+%60unwrap_or%60.%0A%2F%2F+Or+use+an+%60if+let%60+statement+on+the+result+of+%60pop%28%29%60+to+both+destructure%0A%2F%2F+a+%60Some%60+value+and+only+print+out+something+if+we+have+a+value%21%0A)
- ["result1.rs"](https://play.rust-lang.org/?code=%2F%2F+result1.rs%0A%2F%2F+Make+this+test+pass%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60PositiveNonzeroInteger%3A%3Anew%60+is+always+creating+a+new+instance+and+returning+an+%60Ok%60+result.%0A%2F%2F+It+should+be+doing+some+checking%2C+returning+an+%60Err%60+result+if+those+checks+fail%2C+and+only%0A%2F%2F+returning+an+%60Ok%60+result+if+those+checks+determine+that+everything+is...+okay+%3A%29%0A)
- ["errors1.rs"](https://play.rust-lang.org/?code=%2F%2F+errors1.rs%0A%2F%2F+This+function+refuses+to+generate+text+to+be+printed+on+a+nametag+if%0A%2F%2F+you+pass+it+an+empty+string.+It%27d+be+nicer+if+it+explained+what+the+problem%0A%2F%2F+was%2C+instead+of+just+sometimes+returning+%60None%60.+The+2nd+test+currently%0A%2F%2F+does+not+compile+or+pass%2C+but+it+illustrates+the+behavior+we+would+like%0A%2F%2F+this+function+to+have.%0A%2F%2F+Scroll+down+for+hints%21%21%21%0A%0Apub+fn+generate_nametag_text%28name%3A+String%29+-%3E+Option%3CString%3E+%7B%0A++++if+name.len%28%29+%3E+0+%7B%0A++++++++Some%28format%21%28%22Hi%21+My+name+is+%7B%7D%22%2C+name%29%29%0A++++%7D+else+%7B%0A++++++++%2F%2F+Empty+names+aren%27t+allowed.%0A++++++++None%0A++++%7D%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%2F%2F+This+test+passes+initially+if+you+comment+out+the+2nd+test.%0A++++%2F%2F+You%27ll+need+to+update+what+this+test+expects+when+you+change%0A++++%2F%2F+the+function+under+test%21%0A++++%23%5Btest%5D%0A++++fn+generates_nametag_text_for_a_nonempty_name%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++generate_nametag_text%28%22Beyonc%C3%A9%22.into%28%29%29%2C%0A++++++++++++Some%28%22Hi%21+My+name+is+Beyonc%C3%A9%22.into%28%29%29%0A++++++++%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+explains_why_generating_nametag_text_fails%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++generate_nametag_text%28%22%22.into%28%29%29%2C%0A++++++++++++Err%28%22%60name%60+was+empty%3B+it+must+be+nonempty.%22.into%28%29%29%0A++++++++%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60Err%60+is+one+of+the+variants+of+%60Result%60%2C+so+what+the+2nd+test+is+saying%0A%2F%2F+is+that+%60generate_nametag_text%60+should+return+a+%60Result%60+instead+of+an%0A%2F%2F+%60Option%60.%0A%0A%2F%2F+To+make+this+change%2C+you%27ll+need+to%3A%0A%2F%2F+-+update+the+return+type+in+the+function+signature+to+be+a+Result+that%0A%2F%2F+++could+be+the+variants+%60Ok%28String%29%60+and+%60Err%28String%29%60%0A%2F%2F+-+change+the+body+of+the+function+to+return+%60Ok%28stuff%29%60+where+it+currently%0A%2F%2F+++returns+%60Some%28stuff%29%60%0A%2F%2F+-+change+the+body+of+the+function+to+return+%60Err%28error+message%29%60+where+it%0A%2F%2F+++currently+returns+%60None%60%0A%2F%2F+-+change+the+first+test+to+expect+%60Ok%28stuff%29%60+where+it+currently+expects%0A%2F%2F+++%60Some%28stuff%29%60.%0A)
- ["errors2.rs"](https://play.rust-lang.org/?code=%2F%2F+errors2.rs%0A%2F%2F+Say+we%27re+writing+a+game+where+you+can+buy+items+with+tokens.+All+items+cost%0A%2F%2F+5+tokens%2C+and+whenever+you+purchase+items+there+is+a+processing+fee+of+1%0A%2F%2F+token.+A+player+of+the+game+will+type+in+how+many+items+they+want+to+buy%2C%0A%2F%2F+and+the+%60total_cost%60+function+will+calculate+the+total+number+of+tokens.%0A%2F%2F+Since+the+player+typed+in+the+quantity%2C+though%2C+we+get+it+as+a+string--+and%0A%2F%2F+they+might+have+typed+anything%2C+not+just+numbers%21%0A%0A%2F%2F+Right+now%2C+this+function+isn%27t+handling+the+error+case+at+all+%28and+isn%27t%0A%2F%2F+handling+the+success+case+properly+either%29.+What+we+want+to+do+is%3A%0A%2F%2F+if+we+call+the+%60parse%60+function+on+a+string+that+is+not+a+number%2C+that%0A%2F%2F+function+will+return+a+%60ParseIntError%60%2C+and+in+that+case%2C+we+want+to%0A%2F%2F+immediately+return+that+error+from+our+function+and+not+try+to+multiply%0A%2F%2F+and+add.%0A%0A%2F%2F+There+are+at+least+two+ways+to+implement+this+that+are+both+correct--+but%0A%2F%2F+one+is+a+lot+shorter%21+Scroll+down+for+hints+to+both+ways.%0A%0Ause+std%3A%3Anum%3A%3AParseIntError%3B%0A%0Apub+fn+total_cost%28item_quantity%3A+%26str%29+-%3E+Result%3Ci32%2C+ParseIntError%3E+%7B%0A++++let+processing_fee+%3D+1%3B%0A++++let+cost_per_item+%3D+5%3B%0A++++let+qty+%3D+item_quantity.parse%3A%3A%3Ci32%3E%28%29%3B%0A%0A++++Ok%28qty+*+cost_per_item+%2B+processing_fee%29%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+item_quantity_is_a_valid_number%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++total_cost%28%2234%22%29%2C%0A++++++++++++Ok%28171%29%0A++++++++%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+item_quantity_is_an_invalid_number%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++total_cost%28%22beep+boop%22%29.unwrap_err%28%29.to_string%28%29%2C%0A++++++++++++%22invalid+digit+found+in+string%22%0A++++++++%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+One+way+to+handle+this+is+using+a+%60match%60+statement+on%0A%2F%2F+%60item_quantity.parse%3A%3A%3Ci32%3E%28%29%60+where+the+cases+are+%60Ok%28something%29%60+and%0A%2F%2F+%60Err%28something%29%60.+This+pattern+is+very+common+in+Rust%2C+though%2C+so+there%27s%0A%2F%2F+a+%60%3F%60+operator+that+does+pretty+much+what+you+would+make+that+match+statement%0A%2F%2F+do+for+you%21+Take+a+look+at+this+section+of+the+Error+Handling+chapter%3A%0A%2F%2F+https%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fsecond-edition%2Fch09-02-recoverable-errors-with-result.html%23a-shortcut-for-propagating-errors-the--operator%0A%2F%2F+and+give+it+a+try%21%0A)
- ["errors3.rs"](https://play.rust-lang.org/?code=%2F%2F+errors3.rs%0A%2F%2F+This+is+a+program+that+is+trying+to+use+a+completed+version+of+the%0A%2F%2F+%60total_cost%60+function+from+the+previous+exercise.+It%27s+not+working+though--%0A%2F%2F+we+can%27t+use+the+%60%3F%60+operator+in+the+%60main%28%29%60+function%21+Why+not%3F%0A%2F%2F+What+should+we+do+instead%3F+Scroll+for+hints%21%0A%0Ause+std%3A%3Anum%3A%3AParseIntError%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+tokens+%3D+100%3B%0A++++let+pretend_user_input+%3D+%228%22%3B%0A%0A++++let+cost+%3D+total_cost%28pretend_user_input%29%3F%3B%0A%0A++++if+cost+%3E+tokens+%7B%0A++++++++println%21%28%22You+can%27t+afford+that+many%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++tokens+-%3D+cost%3B%0A++++++++println%21%28%22You+now+have+%7B%7D+tokens.%22%2C+tokens%29%3B%0A++++%7D%0A%7D%0A%0Apub+fn+total_cost%28item_quantity%3A+%26str%29+-%3E+Result%3Ci32%2C+ParseIntError%3E+%7B%0A++++let+processing_fee+%3D+1%3B%0A++++let+cost_per_item+%3D+5%3B%0A++++let+qty+%3D+item_quantity.parse%3A%3A%3Ci32%3E%28%29%3F%3B%0A%0A++++Ok%28qty+*+cost_per_item+%2B+processing_fee%29%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Since+the+%60%3F%60+operator+returns+an+%60Err%60+early+if+the+thing+it%27s+trying+to%0A%2F%2F+do+fails%2C+you+can+only+use+the+%60%3F%60+operator+in+functions+that+have+a%0A%2F%2F+%60Result%60+as+their+return+type.%0A%0A%2F%2F+Hence+the+error+that+you+get+if+you+run+this+code+is%3A%0A%0A%2F%2F+%60%60%60%0A%2F%2F+error%5BE0277%5D%3A+the+%60%3F%60+operator+can+only+be+used+in+a+function+that+returns+%60Result%60+%28or+another+type+that+implements+%60std%3A%3Aops%3A%3ATry%60%29%0A%2F%2F+%60%60%60%0A%0A%2F%2F+So+we+have+to+use+another+way+of+handling+a+%60Result%60+within+%60main%60.%0A%0A%2F%2F+Decide+what+we+should+do+if+%60pretend_user_input%60+has+a+string+value+that+does%0A%2F%2F+not+parse+to+an+integer%2C+and+implement+that+instead+of+using+the+%60%3F%60%0A%2F%2F+operator.%0A)
- ["errorsn.rs"](https://play.rust-lang.org/?code=%2F%2F+errorsn.rs%0A%2F%2F+This+is+a+bigger+error+exercise+than+the+previous+ones%21%0A%2F%2F+You+can+do+it%21+%3A%29%0A%2F%2F%0A%2F%2F+Edit+the+%60read_and_validate%60+function+so+that+it+compiles+and%0A%2F%2F+passes+the+tests...+so+many+things+could+go+wrong%21%0A%2F%2F%0A%2F%2F+-+Reading+from+stdin+could+produce+an+io%3A%3AError%0A%2F%2F+-+Parsing+the+input+could+produce+a+num%3A%3AParseIntError%0A%2F%2F+-+Validating+the+input+could+produce+a+CreationError+%28defined+below%29%0A%2F%2F%0A%2F%2F+How+can+we+lump+these+errors+into+one+general+error%3F+That+is%2C+what%0A%2F%2F+type+goes+where+the+question+marks+are%2C+and+how+do+we+return%0A%2F%2F+that+type+from+the+body+of+read_and_validate%3F%0A%2F%2F%0A%2F%2F+Scroll+down+for+hints+%3A%29%0A%0Ause+std%3A%3Aerror%3B%0Ause+std%3A%3Afmt%3B%0Ause+std%3A%3Aio%3B%0A%0A%2F%2F+PositiveNonzeroInteger+is+a+struct+defined+below+the+tests.%0Afn+read_and_validate%28b%3A+%26mut+io%3A%3ABufRead%29+-%3E+Result%3CPositiveNonzeroInteger%2C+%3F%3F%3F%3E+%7B%0A++++let+mut+line+%3D+String%3A%3Anew%28%29%3B%0A++++b.read_line%28%26mut+line%29%3B%0A++++let+num%3A+i64+%3D+line.trim%28%29.parse%28%29%3B%0A++++let+answer+%3D+PositiveNonzeroInteger%3A%3Anew%28num%29%3B%0A++++answer%0A%7D%0A%0A%2F%2F+This+is+a+test+helper+function+that+turns+a+%26str+into+a+BufReader.%0Afn+test_with_str%28s%3A+%26str%29+-%3E+Result%3CPositiveNonzeroInteger%2C+Box%3Cerror%3A%3AError%3E%3E+%7B%0A++++let+mut+b+%3D+io%3A%3ABufReader%3A%3Anew%28s.as_bytes%28%29%29%3B%0A++++read_and_validate%28%26mut+b%29%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_success%28%29+%7B%0A++++let+x+%3D+test_with_str%28%2242%5Cn%22%29%3B%0A++++assert_eq%21%28PositiveNonzeroInteger%2842%29%2C+x.unwrap%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_not_num%28%29+%7B%0A++++let+x+%3D+test_with_str%28%22eleven+billion%5Cn%22%29%3B%0A++++assert%21%28x.is_err%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_non_positive%28%29+%7B%0A++++let+x+%3D+test_with_str%28%22-40%5Cn%22%29%3B%0A++++assert%21%28x.is_err%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_ioerror%28%29+%7B%0A++++struct+Broken%3B%0A++++impl+io%3A%3ARead+for+Broken+%7B%0A++++++++fn+read%28%26mut+self%2C+_buf%3A+%26mut+%5Bu8%5D%29+-%3E+io%3A%3AResult%3Cusize%3E+%7B%0A++++++++++++Err%28io%3A%3AError%3A%3Anew%28io%3A%3AErrorKind%3A%3ABrokenPipe%2C+%22uh-oh%21%22%29%29%0A++++++++%7D%0A++++%7D%0A++++let+mut+b+%3D+io%3A%3ABufReader%3A%3Anew%28Broken%29%3B%0A++++assert%21%28read_and_validate%28%26mut+b%29.is_err%28%29%29%3B%0A++++assert_eq%21%28%22uh-oh%21%22%2C+read_and_validate%28%26mut+b%29.unwrap_err%28%29.to_string%28%29%29%3B%0A%7D%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++if+value+%3D%3D+0+%7B%0A++++++++++++Err%28CreationError%3A%3AZero%29%0A++++++++%7D+else+if+value+%3C+0+%7B%0A++++++++++++Err%28CreationError%3A%3ANegative%29%0A++++++++%7D+else+%7B%0A++++++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++++++%7D%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_positive_nonzero_integer_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+fmt%3A%3ADisplay+for+CreationError+%7B%0A++++fn+fmt%28%26self%2C+f%3A+%26mut+fmt%3A%3AFormatter%29+-%3E+fmt%3A%3AResult+%7B%0A++++++++f.write_str%28%28self+as+%26error%3A%3AError%29.description%28%29%29%0A++++%7D%0A%7D%0A%0Aimpl+error%3A%3AError+for+CreationError+%7B%0A++++fn+description%28%26self%29+-%3E+%26str+%7B%0A++++++++match+*self+%7B%0A++++++++++++CreationError%3A%3ANegative+%3D%3E+%22Negative%22%2C%0A++++++++++++CreationError%3A%3AZero+%3D%3E+%22Zero%22%2C%0A++++++++%7D%0A++++%7D%0A%7D%0A%0A%2F%2F+First+hint%3A+To+figure+out+what+type+should+go+where+the+%3F%3F%3F+is%2C+take+a+look%0A%2F%2F+at+the+test+helper+function+%60test_with_str%60%2C+since+it+returns+whatever%0A%2F%2F+%60read_and_validate%60+returns+and%60test_with_str%60+has+its+signature+fully%0A%2F%2F+specified.%0A%0A%2F%2F+Next+hint%3A+There+are+three+places+in+%60read_and_validate%60+that+we+call+a%0A%2F%2F+function+that+returns+a+%60Result%60+%28that+is%2C+the+functions+might+fail%29.%0A%2F%2F+Apply+the+%60%3F%60+operator+on+those+calls+so+that+we+return+immediately+from%0A%2F%2F+%60read_and_validate%60+if+those+function+calls+fail.%0A%0A%2F%2F+Another+hint%3A+under+the+hood%2C+the+%60%3F%60+operator+calls+%60From%3A%3Afrom%60%0A%2F%2F+on+the+error+value+to+convert+it+to+a+boxed+trait+object%2C+a+Box%3Cerror%3A%3AError%3E%2C%0A%2F%2F+which+is+polymorphic--+that+means+that+lots+of+different+kinds+of+errors%0A%2F%2F+can+be+returned+from+the+same+function+because+all+errors+act+the+same%0A%2F%2F+since+they+all+implement+the+%60error%3A%3AError%60+trait.%0A%2F%2F+Check+out+this+section+of+the+book%3A%0A%2F%2F+https%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fsecond-edition%2Fch09-02-recoverable-errors-with-result.html%23a-shortcut-for-propagating-errors-the--operator%0A%0A%2F%2F+Another+another+hint%3A+Note+that+because+the+%60%3F%60+operator+returns%0A%2F%2F+the+*unwrapped*+value+in+the+%60Ok%60+case%2C+if+we+want+to+return+a+%60Result%60+from%0A%2F%2F+%60read_and_validate%60+for+*its*+success+case%2C+we%27ll+have+to+rewrap+a+value%0A%2F%2F+that+we+got+from+the+return+value+of+a+%60%3F%60ed+call+in+an+%60Ok%60--+this+will%0A%2F%2F+look+like+%60Ok%28something%29%60.%0A%0A%2F%2F+Another+another+another+hint%3A+%60Result%60s+must+be+%22used%22%2C+that+is%2C+you%27ll%0A%2F%2F+get+a+warning+if+you+don%27t+handle+a+%60Result%60+that+you+get+in+your%0A%2F%2F+function.+Read+more+about+that+in+the+%60std%3A%3Aresult%60+module+docs%3A%0A%2F%2F+https%3A%2F%2Fdoc.rust-lang.org%2Fstd%2Fresult%2F%23results-must-be-used%0A)


### Smart Pointers


- ["pointers1.rs"](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=%2F%2F%20make%20it%20compile%0A%0A%0A%23%5Bderive(Debug)%5D%0Astruct%20Node%20%7B%0A%20%20%20%20letft%20%3A%20Option%3CNode%3E%2C%0A%20%20%20%20right%20%3A%20Option%3CNode%3E%0A%7D%0A%23%5Bderive(Debug)%5D%0Astruct%20Tree%20%7B%0A%20%20%20%20root%20%3A%20Option%3CNode%3E%0A%7D%0A%0A%0Afn%20create_tree()%20-%3E%20Tree%20%7B%0A%20%20%20%20Tree%20%7B%20root%20%3A%20None%7D%0A%7D%0Afn%20main()%20%7B%0A%20%20%20%20%0A%20%20%20%20%0A%20%20%20%20let%20tree%20%3D%20create_tree()%3B%0A%20%20%20%20%0A%20%20%20%20println!(%22%7B%3A%3F%7D%22%2C%20tree)%3B%0A%20%20%20%20%0A%7D)

- ["pointers2.rs"](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=%2F%2F%20make%20it%20compile%0A%0A%0A%23%5Bderive(Debug)%5D%0Astruct%20User%20%7B%0A%20%20%20%20address%20%3A%20Address%0A%7D%0A%0A%23%5Bderive(Debug)%5D%0Astruct%20Address%20%7B%0A%20%20%20%20name%20%3A%20String%0A%7D%0A%0A%0A%0A%0Afn%20main()%20%7B%0A%0A%20%20%20%20%2F%2F%20use%20the%20same%20address%20struct%20without%20cloning%20and%20without%20shared%20reference%0A%20%20%20%20let%20address%20%3D%20Address%20%7Bname%20%3A%20String%3A%3Afrom(%22via%20test%22)%7D%3B%0A%20%20%20%20%0A%20%20%20%20let%20user%20%3D%20User%20%7B%20address%20%3A%20address%7D%3B%0A%20%20%20%20let%20user1%20%3D%20User%20%7B%20address%20%3A%20address%7D%3B%0A%20%20%20%20%0A%20%20%20%20%0A%20%20%20%20%0A%20%20%20%20%0A%20%20%20%20println!(%22%7B%3A%3F%7D%20-%20%7B%3A%3F%7D%22%2Cuser%20%2C%20user1)%3B%0A%20%20%20%20%0A%7D)


### Tests

Going out of order from the book to cover tests-- many of the following exercises will ask you to
make tests pass!

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch11-01-writing-tests.html)

- ["tests1.rs"](https://play.rust-lang.org/?code=%2F%2F+tests1.rs%0A%2F%2F+Tests+are+important+to+ensure+that+your+code+does+what+you+think+it+should+do.%0A%2F%2F+Tests+can+be+run+on+this+file+with+the+following+command%3A%0A%2F%2F+rustc+--test+tests1.rs%0A%0A%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert%28%29+%7B%0A++++++++assert%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+don%27t+even+need+to+write+any+code+to+test+--+you+can+just+test+values+and+run+that%2C+even%0A%2F%2F+though+you+wouldn%27t+do+that+in+real+life+%3A%29+%60assert%21%60+is+a+macro+that+needs+an+argument.%0A%2F%2F+Depending+on+the+value+of+the+argument%2C+%60assert%21%60+will+do+nothing+%28in+which+case+the+test+will%0A%2F%2F+pass%29+or+%60assert%21%60+will+panic+%28in+which+case+the+test+will+fail%29.+So+try+giving+different+values%0A%2F%2F+to+%60assert%21%60+and+see+which+ones+compile%2C+which+ones+pass%2C+and+which+ones+fail+%3A%29%0A)
- ["tests2.rs"](https://play.rust-lang.org/?code=%2F%2F+tests2.rs%0A%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert_eq%28%29+%7B%0A++++++++assert_eq%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Like+the+previous+exercise%2C+you+don%27t+need+to+write+any+code+to+get+this+test+to+compile+and%0A%2F%2F+run.+%60assert_eq%21%60+is+a+macro+that+takes+two+arguments+and+compares+them.+Try+giving+it+two%0A%2F%2F+values+that+are+equal%21+Try+giving+it+two+arguments+that+are+different%21+Try+giving+it+two+values%0A%2F%2F+that+are+of+different+types%21+Try+switching+which+argument+comes+first+and+which+comes+second%21%0A)
- ["tests3.rs"](https://play.rust-lang.org/?code=%2F%2F+tests3.rs%0A%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60is_even%285%29%60.+Scroll+down+for+hints%21%0A%0Apub+fn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+is_true_when_even%28%29+%7B%0A++++++++assert%21%28false%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+can+call+a+function+right+where+you%27re+passing+arguments+to+%60assert%21%60+--+so+you+could+do%0A%2F%2F+something+like+%60assert%21%28having_fun%28%29%29%60.+If+you+want+to+check+that+you+indeed+get+false%2C+you%0A%2F%2F+can+negate+the+result+of+what+you%27re+doing+using+%60%21%60%2C+like+%60assert%21%28%21having_fun%28%29%29%60.%0A)
- ["tests4.rs"](https://play.rust-lang.org/?code=%2F%2F+tests4.rs%0A%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60times_two%60+with+a+negative+number.%0A%2F%2F+No+hints%2C+you+can+do+this+%3A%29%0A%0Apub+fn+times_two%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+2%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+returns_twice_of_positive_numbers%28%29+%7B%0A++++++++assert_eq%21%284%2C+4%29%3B%0A++++%7D%0A%7D%0A)