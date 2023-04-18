import { calculate_grade } from "../../v3/pkg/v3.js";

function getRandomInt(max) {
  return Math.floor(Math.random() * Math.floor(max));
}

function getRandomWorkflowState() {
  const states = ["graded", "ungraded", "pending_review"];
  return states[getRandomInt(states.length)];
}

function getRandomSubmission() {
  return {
    id: getRandomInt(1000),
    user_id: 1,
    score: getRandomInt(100),
    points_possible: 100,
    workflow_state: getRandomWorkflowState(),
  };
}

function run() {
  const submissionCount = getRandomInt(100);
  const submissions = Array.from(
    { length: submissionCount },
    getRandomSubmission
  );

  // WASM in NODE!! :D
  const grade = calculate_grade(JSON.stringify(submissions));

  console.log(grade);
};


run();