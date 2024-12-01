#import <Foundation/Foundation.h>

int main(int argc, const char * argv[]) {
        NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

        NSArray *list = [ NSArray arrayWithObjects:@"byr",@"iyr",@"eyr",@"hgt",@"hcl",@"ecl",@"pid",@"cid" ];

        for (NSString *s in list) {
                NSLog(@"%@", s);
                NSInteger i = [ s hash ];
                NSLog(@"%ld", i);
        }
        return 0;
}
