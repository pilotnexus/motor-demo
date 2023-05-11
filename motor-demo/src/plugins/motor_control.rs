
//This file is auto-implemented 2.5.4, do not edit

use pilot_sys::time::wait_next_cycle;

extern "C" {
  fn dc_reset_2(motor_id:u8);
  fn dc_set_current_position_2(motor_id: u8, position: u32);
  fn dc_get_flag_2(motor_id: u8);
  fn dc_set_speed_2(motor_id: u8, speed: u8);
  fn dc_set_target_position_2(motor_id: u8, position: u32);
}

#[allow(dead_code)]
pub async fn dc_reset(motor_id: u8)
{
  if motor_id < 6 {
    unsafe {
      dc_reset_2(motor_id);
    }
    wait_next_cycle().await; //wait for the next PLC cycle
  }
}

#[allow(dead_code)]
pub async fn dc_set_current_position(motor_id: u8, position: u32) 
{
  if motor_id < 6 {
    unsafe {
      dc_set_current_position_2(motor_id, position);
    }
    wait_next_cycle().await; //wait for the next PLC cycle
  }
}

#[allow(dead_code)]
pub async fn dc_get_flag(motor_id: u8)
{
  if motor_id < 6 {
    unsafe {
      dc_get_flag_2(motor_id);
    }
    wait_next_cycle().await; //wait for the next PLC cycle
  }
}

#[allow(dead_code)]
pub async fn dc_move(motor_id: u8, speed: u8, position: u32)
{
  if motor_id < 6 {
    dc_reset(motor_id).await;
    dc_set_speed(motor_id, speed).await;
    dc_set_target_position(motor_id, position).await;
  }
}

#[allow(dead_code)]
pub async fn dc_set_speed(motor_id: u8, speed: u8)
{
  if motor_id < 6 {
    unsafe {
      dc_set_speed_2(motor_id, speed);
    }
    wait_next_cycle().await; //wait for the next PLC cycle
  }
}

#[allow(dead_code)]
pub async fn dc_set_target_position(motor_id: u8, position: u32)
{
  if motor_id < 6 {
    unsafe {
      dc_set_target_position_2(motor_id, position);
    }
    wait_next_cycle().await; //wait for the next PLC cycle
  }
}
