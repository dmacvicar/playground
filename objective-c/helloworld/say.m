#include "say.h"
#include <Foundation/Foundation.h>

@implementation Say
- (void) sayHello
{
   NSLog(@"Hello World");
}

- (void) sayHelloTo: (NSString *)name
{
   NSLog(@"Hello World, %@", name);
}
@end