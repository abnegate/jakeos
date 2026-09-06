# Hardware register

This register names the QEMU profiles and physical machines that milestone gates run on. It is owned by LAB, with SKU selection decided by HW. Entries record kind, tier, CPU, GPU, network, the first milestone whose gates run on the machine, and procurement status. Status is `planned` until a bring-up task lands. Exact SKUs are recorded when the machine is procured; this register never holds measured results.

### H-001 · QEMU/KVM x86-64 CI profile
- Kind: qemu
- Tier: none
- CPU: qemu64, host passthrough in the nightly matrix
- GPU: none
- Network: virtio-net
- First milestone: V0
- Matrix entry: qemu-x86_64
- Provided by: BLD-012
- Status: planned
Primary CI target for the execution-model gates. virtio-blk, virtio-scsi and the QEMU gdbstub are in the profile. No GPU acceleration is required.

### H-002 · AMD reference desktop
- Kind: desktop
- Tier: 1
- CPU: Zen 4-class
- GPU: RDNA 3-class discrete
- Network: wired Ethernet
- First milestone: V0
- Matrix entry: hw-h002
- Provided by: LAB-003
- Status: planned
The single V0 physical machine. NVMe boot, one display at fixed refresh for V0 and V0.5. At V2 this machine gains the HDR/VRR reference display and colorimeter owned by LAB. IOMMU, TPM 2.0 and Secure Boot custom-key enrolment are selection criteria.

### H-003 · QEMU/KVM x86-64 with virtio-gpu
- Kind: qemu
- Tier: none
- CPU: qemu64, host passthrough in the nightly matrix
- GPU: virtio-gpu
- Network: virtio-net
- First milestone: V0.5
- Matrix entry: qemu-virtio-gpu
- Provided by: BLD-012, BLD-028
- Status: planned
Compositor CI profile. Used for crash-rebind loops and the four-application scenarios that do not need a physical scanout.

### H-004 · Intel reference laptop
- Kind: laptop
- Tier: 1
- CPU: Intel with integrated GPU
- GPU: Intel integrated
- Network: Intel Wi-Fi, wired Ethernet via dock
- First milestone: V1
- Matrix entry: hw-h004
- Provided by: LAB-007
- Status: planned
Procured at V0.5 so V1 Wi-Fi roaming, suspend and battery gates have a machine. Internal display plus one external display over USB-C. Bluetooth and HDR/VRR are not required to work at V1. TPM 2.0, IOMMU and Secure Boot enrolment are selection criteria.

### H-005 · AMD reference laptop
- Kind: laptop
- Tier: 1
- CPU: Zen 4-class APU
- GPU: AMD integrated
- Network: Wi-Fi 6, Bluetooth, wired Ethernet via dock
- First milestone: V2
- Matrix entry: hw-h005
- Provided by: LAB-018
- Status: planned
Second laptop of the V2 three-machine set. Battery, thermal, lid, brightness and suspend/resume gates run here.

### H-006 · NVIDIA desktop
- Kind: desktop
- Tier: 1
- CPU: x86-64 desktop class
- GPU: NVIDIA discrete, proprietary or open kernel-module path per the HW decision
- Network: wired Ethernet
- First milestone: V3
- Matrix entry: hw-h006
- Provided by: LAB-021
- Status: planned
Tracked as experimental at V2 with no gate. Becomes a Tier 1 gated machine at V3. The NVIDIA driver and Secure Boot module-signing decisions land before this machine's first gate.

### H-007 · Intel laptop, second generation
- Kind: laptop
- Tier: 1
- CPU: Intel, one generation apart from H-004
- GPU: Intel integrated
- Network: Intel Wi-Fi, Bluetooth
- First milestone: V3
- Matrix entry: hw-h007
- Provided by: LAB-021
- Status: planned
The additional Intel laptop that takes Tier 1 from three machines to six at V3. SKU selection requires an NPU with an upstream Linux driver so HET-020 has a reference ComputeDevice backend.

### H-008 · AMD laptop, second generation
- Kind: laptop
- Tier: 1
- CPU: Zen-class APU, one generation apart from H-005
- GPU: AMD integrated
- Network: Wi-Fi 6, Bluetooth
- First milestone: V3
- Matrix entry: hw-h008
- Provided by: LAB-021
- Status: planned
The additional AMD laptop of the V3 six-machine Tier 1 set.

### H-009 · AMD desktop, second GPU generation
- Kind: desktop
- Tier: 1
- CPU: Zen-class
- GPU: RDNA class, one generation apart from H-002
- Network: wired Ethernet
- First milestone: V4
- Matrix entry: hw-h009
- Provided by: LAB-023
- Status: planned
Satisfies the V4 requirement for AMD desktops covering at least two GPU generations.

### H-010 · Intel desktop
- Kind: desktop
- Tier: 1
- CPU: Intel desktop class
- GPU: Intel discrete or integrated as the SKU provides
- Network: wired Ethernet
- First milestone: V4
- Matrix entry: hw-h010
- Provided by: LAB-023
- Status: planned
The Intel desktop that V4 adds to Tier 1.

### H-011 · Intel hybrid-graphics laptop
- Kind: laptop
- Tier: 1
- CPU: Intel with integrated GPU
- GPU: Intel integrated plus NVIDIA discrete, render offload and mux switching
- Network: Intel Wi-Fi, Bluetooth
- First milestone: V4
- Matrix entry: hw-h011
- Provided by: LAB-023
- Status: planned
Exercises hybrid graphics across the compositor and both personalities. Counts toward the V4 ten-machine Tier 1 floor.

### H-012 · AMD hybrid-graphics laptop
- Kind: laptop
- Tier: 1
- CPU: Zen-class APU
- GPU: AMD integrated plus discrete, render offload
- Network: Wi-Fi 6, Bluetooth
- First milestone: V4
- Matrix entry: hw-h012
- Provided by: LAB-023
- Status: planned
Second hybrid-graphics laptop. Counts toward the V4 ten-machine Tier 1 floor.

### H-013 · Intel laptop, third named SKU
- Kind: laptop
- Tier: 1
- CPU: Intel, a third named SKU distinct from H-004, H-007 and H-011
- GPU: Intel integrated
- Network: Intel Wi-Fi, Bluetooth
- First milestone: V4
- Matrix entry: hw-h013
- Provided by: LAB-023
- Status: planned
Fills the V4 ten-machine floor and the "Intel laptops, at least two generations" rule with margin.

### H-014 · AMD desktop, entry discrete
- Kind: desktop
- Tier: 1
- CPU: Zen-class
- GPU: AMD discrete, a third named SKU distinct from H-002 and H-009
- Network: wired Ethernet
- First milestone: V4
- Matrix entry: hw-h014
- Provided by: LAB-023
- Status: planned
Third AMD desktop SKU so the lab is not a two-machine sample of the vendor.

### H-015 · QEMU/KVM nested virtualization profile
- Kind: qemu
- Tier: none
- CPU: qemu64 with nested KVM
- GPU: virtio-gpu
- Network: virtio-net
- First milestone: V1
- Matrix entry: qemu-nested
- Provided by: BLD-012, KRN-036
- Status: planned
CI profile for the VIRT VM-manager component, guest tools and JakeOS-as-guest images.

### H-016 · QEMU/KVM ia32 userspace profile
- Kind: qemu
- Tier: none
- CPU: qemu64 with ia32 emulation enabled
- GPU: none
- Network: virtio-net
- First milestone: V1
- Matrix entry: qemu-ia32
- Provided by: BLD-012
- Status: planned
Holds the 32-bit decision in CI so syscall pruning cannot delete ia32 before Steam and Windows titles need it.

### H-017 · QEMU/KVM VFIO GPU passthrough profile
- Kind: qemu
- Tier: none
- CPU: qemu64, host passthrough
- GPU: VFIO assignment of a lab discrete GPU
- Network: virtio-net
- First milestone: V2
- Matrix entry: qemu-vfio
- Provided by: BLD-012
- Status: planned
Used for Windows-personality GPU and HDR bring-up when a physical dual-boot is not the right isolation.

### H-018 · Community Tier 2 sample desktop
- Kind: desktop
- Tier: 2
- CPU: x86-64, community-reported
- GPU: as reported
- Network: as reported
- First milestone: none
- Matrix entry: none
- Provided by: none
- Status: planned
Placeholder for the first community-submitted Hardware Compatibility List entry. Best-effort; the installer warns. Promotion to Tier 1 is a documented REL process, not an automatic gate. CI matrix entry: none; community machines never appear in the lab matrix.
