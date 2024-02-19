// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           28
// Async Callback (empty):               1
// Total number of exported functions:  30

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    growth_program
    (
        init => init
        upgrade => upgrade
        addProject => add_project
        setProjectOwner => set_project_owner
        setMinRewardsPeriod => set_min_rewards_period
        setMinWeeklyRewardsValue => set_min_weekly_rewards_value
        depositInitialRewards => deposit_initial_rewards
        depositAdditionalRewards => deposit_additional_rewards
        ownerWithdrawRewards => owner_withdraw_rewards
        finishProgram => finish_program
        setMinRewardDollarsPerEnergy => set_min_reward_dollars_per_energy
        setNextWeekRewardDollarsPerEnergy => set_next_week_reward_dollars_per_energy
        setAlpha => set_alpha
        setBeta => set_beta
        claimRewards => claim_rewards
        getExemptedParticipants => get_exempted_participants
        getUserClaimed => get_user_claimed
        updateRewards => update_rewards_endpoint
        getRewardsInfo => rewards_info
        getRewardsTotalAmount => rewards_total_amount
        getRewardsRemainingAmount => rewards_remaining_amount
        changeSigner => change_signer
        getCurrentWeek => get_current_week
        getFirstWeekStartTimestamp => first_week_start_timestamp
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
