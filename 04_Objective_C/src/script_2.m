#import <Foundation/Foundation.h>

int main(int argc, const char * argv[]) {
        NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

        // Read file content
        if (argc < 2) {
            NSLog(@"\rUsage: %s <filename>", argv[0]);
            return 1;
        }

        NSString *filePath = [NSString stringWithUTF8String:argv[1]];
        NSData *fileData = [NSData dataWithContentsOfFile:filePath];
        if (!fileData) {
            NSLog(@"\rCould not read file %@", [NSString stringWithUTF8String:argv[1]]);
            return 1;
        }

        NSString *fileContent = [[NSString alloc] initWithData:fileData encoding:NSUTF8StringEncoding];
        if (!fileContent) {
            NSLog(@"\rCould not decode file content");
            return 1;
        }

        NSLog(@"\r%@", fileContent);

        // Get the passports (passports are separated by a blank line: \n\n)
        NSMutableArray *passports = [NSMutableArray arrayWithArray:[fileContent componentsSeparatedByString:@"\n\n"]];
        NSLog(@"\rPassports: %@\n", passports);

        NSInteger count = 0;

        NSMutableArray *parsedPassports = [NSMutableArray array];
        for (int i = 0; i < [passports count]; i++) {
                // replace \n with space in passports
                NSString *passport = [passports objectAtIndex:i];
                passport = [passport stringByReplacingOccurrencesOfString:@"\n" withString:@" "];
                NSLog(@"\rPassport %d: %@", i, passport);

                // split passport on spaces
                NSArray *passportFields = [passport componentsSeparatedByString:@" "];
                NSLog(@"\rPassport fields: %@", passportFields);

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
                        NSLog(@"\rValid passport: %@", passportDict);
                        count += 1;
                }

                NSLog(@"\rPassport dictionary: %@", passportDict);

                [parsedPassports addObject:passportDict];
        }

        NSLog(@"\rParsed passports: %@", parsedPassports);
        NSLog(@"\r---\nValid passports: %d\n", count);

        // Exit program
        [pool drain];
        return 0;
}

