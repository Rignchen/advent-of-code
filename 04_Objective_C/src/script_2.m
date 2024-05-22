#import <Foundation/Foundation.h>

int main(int argc, const char * argv[]) {
        // blank = \r + 40 spaces + \r
        NSString *blank = @"\r                                        \r";
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
                if (valid) {
                        NSLog(@"%@Valid passport: %@", blank, passportDict);
                        count += 1;
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

