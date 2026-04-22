/**
 * 保存Trap上下文，类似函数调用上下文，即在 Trap 发生时需要保存的物理资源内容，
 * 并将其一起放在一个名为 TrapContext 的类型中
 * 包含所有的通用寄存器 x0~x31 ，还有 sstatus 和 sepc
 */

use riscv::register::sstatus::{self, SPP, Sstatus};

 #[repr(C)]
pub struct TrapContext{
    /// general regs[0..31]
    pub x: [usize; 32],
    // CSR sstatus
    pub sstaus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp }
    
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self{
            x: [0; 32],
            sstaus: sstatus,
            sepc: entry
        };
        cx.set_sp(sp);
        cx
    }
}