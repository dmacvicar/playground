#ifndef _Say_H_
#define _Say_H_

#include <Foundation/NSObject.h>
@interface Say: NSObject
{
}
- (void) sayHello;
- (void) sayHelloTo: (NSString *)name;
@end

#endif /* _Say_H_ */