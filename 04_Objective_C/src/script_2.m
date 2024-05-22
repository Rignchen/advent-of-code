#import <Foundation/Foundation.h>

void pause() {
        NSString *blank = @"\r                                        \r";
        NSLog(@"%@Press enter to continue...", blank);
        getchar();
}

int main(int argc, const char * argv[]) {
        // blank = \r + 40 spaces + \r
        NSString *blank = @"\r                                        \r";
        pause();
        NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

        // Read file content
        if (argc < 2) {
            NSLog(@"%@Usage: %s <filename>", blank, argv[0]);
            return 1;
        }

        NSString *filePath = [NSString stringWithUTF8String:argv[1]];
        NSData *fileData = [NSData dataWithContentsOfFile:filePath];
        if (!fileData) {
            NSLog(@"%@Could not read file %@", blank, [NSString stringWithUTF8String:argv[1]]);
            return 1;
        }

        NSString *fileContent = [[NSString alloc] initWithData:fileData encoding:NSUTF8StringEncoding];
        if (!fileContent) {
            NSLog(@"%@Could not decode file content", blank);
            return 1;
        }

        NSLog(@"%@%@", blank, fileContent);

        pause();
        // Get the passports (passports are separated by a blank line: \n\n)
        NSMutableArray *passports = [NSMutableArray arrayWithArray:[fileContent componentsSeparatedByString:@"\n\n"]];
        NSLog(@"%@Passports: %@\n", blank, passports);

        NSInteger count = 0;

        NSMutableArray *parsedPassports = [NSMutableArray array];
        for (int i = 0; i < [passports count]; i++) {
                // replace \n with space in passports
                NSString *passport = [passports objectAtIndex:i];
                passport = [passport stringByReplacingOccurrencesOfString:@"\n" withString:@" "];
                NSLog(@"%@Passport %d: %@", blank, i, passport);

                // split passport on spaces
                NSArray *passportFields = [passport componentsSeparatedByString:@" "];
                NSLog(@"%@Passport fields: %@", blank, passportFields);

                // remove empty fields
                // create dictionary from passport fields (passwords fields are in format key:value)
                NSMutableDictionary *passportDict = [NSMutableDictionary dictionary];
                NSMutableArray *keys = [NSMutableArray array];
                for (int j = 0; j < [passportFields count]; j++) {
                        NSString *field = [passportFields objectAtIndex:j];
                        if ([field length] > 0) {
                                NSArray *fieldComponents = [field componentsSeparatedByString:@":"];
                                [passportDict setObject:[fieldComponents objectAtIndex:1] forKey:[fieldComponents objectAtIndex:0]];
                                [keys addObject:[fieldComponents objectAtIndex:0]];
                        }
                }

                // check if all required fields are present
                // byr iyr eyr hgt hcl ecl pid cid 
                NSArray *requiredFields = [NSArray arrayWithObjects:@"byr", @"iyr", @"eyr", @"hgt", @"hcl", @"ecl", @"pid", nil];
                BOOL valid = YES;
                for (int k = 0; k < [requiredFields count]; k++) {
                        if (![keys containsObject:[requiredFields objectAtIndex:k]]) {
                                valid = NO;
                                break;
                        }
                }

                for (int k = 0; k < [keys count]; k++) {
                        NSString *key = [keys objectAtIndex:k];
                        NSString *value = [passportDict objectForKey:key];
                        switch([key hash]) {
                                case 81524269: // byr
                                        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
                                        if (value.length != 4 || [value intValue] < 1920 || [value intValue] > 2002) {
                                                NSLog(@"%@Invalid byr: %@", blank, value);
                                                valid = NO;
                                        }
                                        break;
                                case 87036564 : // iyr
                                        //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                                        if (value.length != 4 || [value intValue] < 2010 || [value intValue] > 2020) {
                                                NSLog(@"%@Invalid iyr: %@", blank, value);
                                                valid = NO;
                                        }
                                        break;
                                case 198930448 : // eyr
                                        //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
if (value.length != 4 || [value intValue] < 2020 || [value intValue] > 2030) {
                                                NSLog(@"%@Invalid eyr: %@", blank, value);
                                                valid = NO;
                                        }
break;
                                case 47254371 : // hgt
                                        //hgt (Height) - a number followed by either cm or in:
                                                //        If cm, the number must be at least 150 and at most 193.
                                                //        If in, the number must be at least 59 and at most 76.
                                        break;
                                case 47110359 : // hcl
                                        //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                                        NSString *pattern = @"^#[0-9a-f]{6}$";
                                        NSRegularExpression* regex = [NSRegularExpression regularExpressionWithPattern: pattern options:0 error:nil];
                                        NSUInteger matches = [regex numberOfMatchesInString:value options:0 range:NSMakeRange(0, [value length])];
                                        if (matches == 0) {
                                                NSLog(@"%@Invalid hcl: %@", blank, value);
                                                valid = NO;
                                        }
                                        break;
                                case 198139636 : // ecl
                                        //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                                        break;
                                case 91973405 : // pid
                                        //pid (Passport ID) - a nine-digit number, including leading zeroes.
                                       break;
                                case 120084208 : // cid
                                        //cid (Country ID) - ignored, missing or not.
                                        break;
                        }
                }

                if (valid) {
                        NSLog(@"%@Valid passport: %@", blank, passportDict);
                        count += 1;
                }
                else {
                        pause();
                }

                NSLog(@"%@Passport dictionary: %@", blank, passportDict);

                [parsedPassports addObject:passportDict];
        }

        NSLog(@"%@Parsed passports: %@", blank, parsedPassports);
        NSLog(@"%@---\nValid passports: %d\n", blank, count);

        // Exit program
        [pool drain];
        return 0;
}

