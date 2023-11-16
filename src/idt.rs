use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        // idt.coprocessor_segment_overrun.set_handler_fn(coprocessor_segment_overrun_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        // idt.reserved_1.set_handler_fn(reserved_1_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.cp_protection_exception.set_handler_fn(cp_protection_exception_handler);
        // idt.reserved_2.set_handler_fn(reserved_2_handler);
        idt.hv_injection_exception.set_handler_fn(hv_injection_exception_handler);
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        // idt.reserved_3.set_handler_fn(reserved_3_handler);
        // idt.interrupts.set_handler_fn(interrupts_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: NON MASKABLE INTERRUPT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// extern "x86-interrupt" fn coprocessor_segment_overrun_handler(stack_frame: InterruptStackFrame) {
//     println!("EXCEPTION: COPROCESSOR SEGMENT OVERRUN\n{:#?}", stack_frame);
// }

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: INVALID TSS\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: STACK SEGMENT FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, _error_code: PageFaultErrorCode) {
    println!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

// extern "x86-interrupt" fn reserved_1_handler(stack_frame: InterruptStackFrame) {
//     println!("EXCEPTION: RESERVED 1\n{:#?}", stack_frame);
// }

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: X87 FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn cp_protection_exception_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: CP PROTECTION EXCEPTION\n{:#?}", stack_frame);
}

// extern "x86-interrupt" fn reserved_2_handler(stack_frame: InterruptStackFrame) {
//     println!("EXCEPTION: RESERVED 2\n{:#?}", stack_frame);
// }

extern "x86-interrupt" fn hv_injection_exception_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: HV INJECTION EXCEPTION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn vmm_communication_exception_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: VMM COMMUNICATION EXCEPTION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: SECURITY EXCEPTION\n{:#?}", stack_frame);
}

// extern "x86-interrupt" fn reserved_3_handler(stack_frame: InterruptStackFrame) {
//     println!("EXCEPTION: RESERVED 3\n{:#?}", stack_frame);
// }

// extern "x86-interrupt" fn interrupts_handler(stack_frame: InterruptStackFrame) {
//     println!("EXCEPTION: INTERRUPTS\n{:#?}", stack_frame);
// }