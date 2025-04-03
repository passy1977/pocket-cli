mod constants;
mod services;
mod models;

use services::cli::Cli;

    /**
     * ADD_USER|test@test.com|pwd|user
     * MOD_USER|test@test.com|pwd1|user1
     * RM_USER|test@test.com
     * GET_USER|test@test.com
     *
     * ADD_DEVICE|test@test.com
     * RM_DEVICE|test@test.com|47a48e92-c521-4f07-a4b3-757c889a0816
     * GET_DEVICE|test@test.com|47a48e92-c521-4f07-a4b3-757c889a0816
     */


fn main() {
     let _ = Cli::perform();


}