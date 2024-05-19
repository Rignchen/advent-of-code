#import <Foundation/Foundation.h>

int main(int argc, const char * argv[]) {
        NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];


        if (argc < 2) {
            NSLog(@"Usage: %s <filename>", argv[0]);
            return 1;
        }

        NSString *filePath = [NSString stringWithUTF8String:argv[1]];
        NSData *fileData = [NSData dataWithContentsOfFile:filePath];
        if (!fileData) {
            NSLog(@"Could not read file %@", [NSString stringWithUTF8String:argv[1]]);
            return 1;
        }

        NSString *fileContent = [[NSString alloc] initWithData:fileData encoding:NSUTF8StringEncoding];
        if (!fileContent) {
            NSLog(@"Could not decode file content");
            return 1;
        }

        NSLog(@"%@", fileContent);

        [pool drain];
        return 0;
}

