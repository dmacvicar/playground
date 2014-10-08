
#include <llvm/IR/Module.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/PassManager.h>
#include <llvm/IR/CallingConv.h>
#include <llvm/Analysis/Verifier.h>
#include <llvm/Assembly/PrintModulePass.h>

using namespace llvm;

Module* makeLLVMModule();

Module* makeLLVMModule() {
  // Module Construction
  Module* mod = new Module("test");
 Constant* c = mod->getOrInsertFunction("mul_add",
  /*ret type*/                           IntegerType::get(32),
  /*args*/                               IntegerType::get(32),
                                         IntegerType::get(32),
                                         IntegerType::get(32),
  /*varargs terminated with null*/       NULL);
  
  Function* mul_add = cast<Function>(c);
  mul_add->setCallingConv(CallingConv::C);
  Function::arg_iterator args = mul_add->arg_begin();
  Value* x = args++;
  x->setName("x");
  Value* y = args++;
  y->setName("y");
  Value* z = args++;
  z->setName("z");
  BasicBlock* block = BasicBlock::Create("entry", mul_add);
  IRBuilder<> builder(block);

Value* tmp = builder.CreateBinOp(Instruction::Mul,
                                   x, y, "tmp");
  Value* tmp2 = builder.CreateBinOp(Instruction::Add,
                                    tmp, z, "tmp2");

  builder.CreateRet(tmp2);
  
  return mod;
}


int main(int argc, char**argv) {
  Module* Mod = makeLLVMModule();

  verifyModule(*Mod, PrintMessageAction);

  PassManager PM;
  PM.add(new PrintModulePass(&llvm::cout));
  PM.run(*Mod);

  delete Mod;
  return 0;
}
