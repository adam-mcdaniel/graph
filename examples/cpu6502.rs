use std::path::Path;
use graph::{Graph, ID};

fn main() {
    cpu_6502_diagram();
}

fn cpu_6502_diagram() {
    let mut g = Graph::new()
        .with_property("rankdir", "TB")
        .with_property("nodesep", "0.5")
        .with_property("layout", "fdp");

    let cpu = g.new_record("CPU_6502", 5, 20)
        .insert(2, 0, "6502\\nCPU")
        .insert(1, 0, "1")
        .insert(1, 1, "2")
        .insert(1, 2, "3")
        .insert(1, 3, "4")
        .insert(1, 4, "5")
        .insert(1, 5, "6")
        .insert(1, 6, "7")
        .insert(1, 7, "8")
        .insert(1, 8, "9")
        .insert(1, 9, "10")
        .insert(1, 10, "11")
        .insert(1, 11, "12")
        .insert(1, 12, "13")
        .insert(1, 13, "14")
        .insert(1, 14, "15")
        .insert(1, 15, "16")
        .insert(1, 16, "17")
        .insert(1, 17, "18")
        .insert(1, 18, "19")
        .insert(1, 19, "20")

        .insert(0, 0, "VSS")
        .insert(0, 1, "RDY")
        .insert(0, 2, "1 (OUT)")
        .insert(0, 3, "IRQ")
        .insert(0, 4, "N.C.")
        .insert(0, 5, "NMI")
        .insert(0, 6, "SYNC")
        .insert(0, 7, "VCC")
        .insert(0, 8, "A0")
        .insert(0, 9, "A1")
        .insert(0, 10, "A2")
        .insert(0, 11, "A3")
        .insert(0, 12, "A4")
        .insert(0, 13, "A5")
        .insert(0, 14, "A6")
        .insert(0, 15, "A7")
        .insert(0, 16, "A8")
        .insert(0, 17, "A9")
        .insert(0, 18, "A10")
        .insert(0, 19, "A11")

        .insert(3, 0, "40")
        .insert(3, 1, "39")
        .insert(3, 2, "38")
        .insert(3, 3, "37")
        .insert(3, 4, "36")
        .insert(3, 5, "35")
        .insert(3, 6, "34")
        .insert(3, 7, "33")
        .insert(3, 8, "32")
        .insert(3, 9, "31")
        .insert(3, 10, "30")
        .insert(3, 11, "29")
        .insert(3, 12, "28")
        .insert(3, 13, "27")
        .insert(3, 14, "26")
        .insert(3, 15, "25")
        .insert(3, 16, "24")
        .insert(3, 17, "23")
        .insert(3, 18, "22")
        .insert(3, 19, "21")

        .insert(4, 0, "RES")
        .insert(4, 1, "2 (OUT)")
        .insert(4, 2, "S.O.")
        .insert(4, 3, "0 (IN)")
        .insert(4, 4, "N.C.")
        .insert(4, 5, "N.C.")
        .insert(4, 6, "R/W")
        .insert(4, 7, "D0")
        .insert(4, 8, "D1")
        .insert(4, 9, "D2")
        .insert(4, 10, "D3")
        .insert(4, 11, "D4")
        .insert(4, 12, "D5")
        .insert(4, 13, "D6")
        .insert(4, 14, "D7")
        .insert(4, 15, "A15")
        .insert(4, 16, "A14")
        .insert(4, 17, "A13")
        .insert(4, 18, "A12")
        .insert(4, 19, "VSS")
        .finalize();
    
    let a = g.new_node("Complex Wiring Stuff")
        .with_property("label", "Complex Wiring Stuff")
        .with_property("pos", "-6,0!")
        .finalize();
    let b = g.new_node("Other Complex Wiring Stuff")
        .with_property("label", "Other Complex Wiring Stuff")
        .with_property("pos", "6,0!")
        .finalize();

    for y in 0..20 {
        g.new_edge(cpu[4][y], b)
            .finalize();

        g.new_edge(a, cpu[0][y])
            .finalize();
    }

    g.build_svg(&Path::new("cpu_6502.svg"));
}

