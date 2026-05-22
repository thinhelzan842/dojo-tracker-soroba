#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
pub struct Student {
    pub attendance_count: u32,
    pub belt_level: u32, 
    pub total_fees_paid: i128,
}

#[contract]
pub struct DojoContract;

const ADMIN: Symbol = Symbol::short("ADMIN");
const PROMOTION_THRESHOLD: u32 = 24; // Đủ 24 buổi thì lên đai

#[contractimpl]
impl DojoContract {
    // 1. Khởi tạo Admin
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN, &admin);
    }

    // 2. Thêm môn sinh mới
    pub fn add_student(env: Env, admin: Address, student_addr: Address) {
        admin.require_auth(); 
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        assert!(admin == stored_admin, "Chi Admin moi duoc them mon sinh");

        let student = Student {
            attendance_count: 0,
            belt_level: 0, // Bắt đầu ở đai trắng (0)
            total_fees_paid: 0,
        };
        env.storage().persistent().set(&student_addr, &student);
    }

    // 3. Điểm danh & Tự động xét thăng đai
    pub fn attend(env: Env, admin: Address, student_addr: Address) {
        admin.require_auth();
        let mut student: Student = env.storage().persistent().get(&student_addr).unwrap();
        
        student.attendance_count += 1;

        // Logic tự động thăng đai
        if student.attendance_count % PROMOTION_THRESHOLD == 0 {
            student.belt_level += 1;
        }

        env.storage().persistent().set(&student_addr, &student);
    }

    // 4. Thanh toán học phí
    pub fn pay_fee(env: Env, student_addr: Address, token_addr: Address, amount: i128) {
        student_addr.require_auth(); 
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        
        token_client.transfer(&student_addr, &admin, &amount);

        let mut student: Student = env.storage().persistent().get(&student_addr).unwrap();
        student.total_fees_paid += amount;
        env.storage().persistent().set(&student_addr, &student);
    }

    // 5. Tra cứu thông tin
    pub fn get_student(env: Env, student_addr: Address) -> Student {
        env.storage().persistent().get(&student_addr).unwrap()
    }
}