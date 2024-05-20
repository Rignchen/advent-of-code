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

        // 

        // Exit program
        [pool drain];
        return 0;
}

