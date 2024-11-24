const WEBSOCKET_URL: string = 'ws://localhost:8080/ws';

const BASE_PATH: string = "http://localhost:8081/api"

let SCENARIO_ID: string = "";

function setScenarioId(scenarioId: string) {
    SCENARIO_ID = scenarioId;
}

let SOLVER: string = "nearest";

function setSolver(solver: string) {
    SOLVER = solver;
}

const MUNICH_X_MIN: number = 48.113000;
const MUNICH_X_MAX: number = 48.165312;
const MUNICH_Y_MIN: number = 11.503302;
const MUNICH_Y_MAX: number = 11.646708;
const MUNICH_X_CENTER: number = (MUNICH_X_MIN + MUNICH_X_MAX) / 2;
const MUNICH_Y_CENTER: number = (MUNICH_Y_MIN + MUNICH_Y_MAX) / 2;

export {
    WEBSOCKET_URL,
    MUNICH_X_MIN,
    MUNICH_X_MAX,
    MUNICH_Y_MIN,
    MUNICH_Y_MAX,
    MUNICH_X_CENTER,
    MUNICH_Y_CENTER,
    BASE_PATH,
    SCENARIO_ID,
    setScenarioId,
    setSolver,
    SOLVER
};