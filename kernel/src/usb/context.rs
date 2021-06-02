#[repr(C, align(32))]
struct SlotContext {
    data: [u32; 8],
}

#[repr(C, align(32))]
struct EndpointContext {
    data: [u32; 8],
}

#[repr(C, align(64))]
struct DeviceContext {
    slot_context: SlotContext,
    endpoint_contexts: [EndpointContext; 31],
}
