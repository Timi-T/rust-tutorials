import { useLocation, useNavigate } from "react-router-dom";
import { ReactComponent as ProjectIcon } from "../../assets/svg/icons/dedicated-project.svg";
import { ReactComponent as Back } from "../../assets/svg/icons/back.svg";
import { ReactComponent as Pause } from "../../assets/svg/icons/blue-pause.svg";
import { ReactComponent as Play } from "../../assets/svg/icons/blue-play.svg";
import { ReactComponent as Bin } from "../../assets/svg/icons/tiny-bin.svg";
import { ReactComponent as Stop } from "../../assets/svg/icons/blue-stop.svg";
import { ReactComponent as Copy } from "../../assets/svg/icons/content_copy.svg";
import { ReactComponent as DateIcon } from "../../assets/svg/icons/date-icon.svg";
import Loader from "../../assets/img/loader.gif";
import { useEffect, useState } from "react";
import {
  PauseModal,
  ResumeModal,
  TerminateModal,
} from "../../components/nodes/Modals";
import { parachainProjects, projects } from "../../components/nodes/DummyData";
import CPUChart from "../../components/nodes/charts/CPUChart";
import { getUnit } from "../../components/nodes/dedicated-nodes/02-step";
import {
  NETWORK_STATE,
  alertError,
  capitalizeString,
  convertToTeraByte,
  getDateTime,
  getTimeLength,
} from "../../utils";
import { useAuth } from "../../context/authContext";
import {
  getDedicatedProject,
  getParachainProject,
} from "../../services/ProjectsService";
import ParachainCPUChart, {
  nodeColors,
} from "../../components/nodes/charts/ParachainCpuChart";
import { useLiveUpdates } from "../../context/liveUpdates";
import { appConfig } from "../../config";
import { StatusDiv } from "../../components/nodes/parachain-nodes/ProjectRow";
import axiosApiInstance from "../../axios";
import { networkIcons } from "../../components/api-services/Networks";

const LiveUptime = ({ startDate }) => {
  const [uptime, setUptime] = useState("");

  useEffect(() => {
    setInterval(() => {
      setUptime(getTimeLength(startDate));
    }, 1000);
  }, []);

  return <>{uptime}</>;
};

const ParachainProjectInsight = () => {
  const navigate = useNavigate();
  const [pauseModalIsOpen, setPauseModalIsOpen] = useState(false);
  const [resumeModalIsOpen, setResumeModalIsOpen] = useState(false);
  const [terminateModalIsOpen, setTerminateModalIsOpen] = useState(false);
  const [liveProjectStatus, setLiveProjectStatus] = useState("active");
  const location = useLocation();
  const [project, setProject] = useState(null);
  const auth = useAuth();
  const [status, setStatus] = useState(NETWORK_STATE.LOADING);
  const [isCopied, setIsCopied] = useState(false);
  const [nodeTypes, setNodeTypes] = useState([
    "validator",
    "archival",
    "full",
    "boot",
  ]);
  const [activeNodeType, setActiveNodeType] = useState("");
  const [parachainNodes, setParachainNodes] = useState([]);
  const [cloudProvider, setCloudProvider] = useState(null);
  const { lastEvent, reloadParachainList, setReloadParachainList } =
    useLiveUpdates();
  const selectionTabs = ["Overview", "Graph", "Nodes", "Endpoints"];
  const [activeTab, setActiveTab] = useState(selectionTabs[0]);

  const response = [
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "boot",
      count: 2,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 8,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "7362087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "collator",
      count: 1,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 6,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "7362087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "validator",
      count: 5,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 8,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "7362087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
  ];

  const fetchProject = async () => {
    const pathSplit = location.pathname.split("/");
    const parachainId = pathSplit[pathSplit.length - 1];

    setStatus(NETWORK_STATE.LOADING);
    try {
      //const response = await getParachainProject(parachainId);
      //const userNodes = response.data?.data;
      const userNodes = response;
      auth.setHeaderText(["Parachain Nodes", userNodes[0]?.projectName]);
      //const userNodes = parachainProjects[0];
      const allNodeTypes = {};
      userNodes.forEach((node) => {
        const nodeType = node.nodeType.split(" ")[0];
        if (!allNodeTypes[nodeType]) {
          allNodeTypes[nodeType] = [node];
        } else {
          allNodeTypes[nodeType].push(node);
        }
      });
      //console.log(allNodeTypes);
      setActiveNodeType(Object.keys(allNodeTypes)[0]);
      setParachainNodes(allNodeTypes);
      const cProvider = appConfig.cloudProviders.filter((provider) => {
        return provider.slug === userNodes[0].cloudProvider;
      });
      setCloudProvider(cProvider[0]);
      setStatus(NETWORK_STATE.SUCCESS);
      /* setLiveProjectStatus(userProject?.status);
      userProject && setProject(userProject);
      auth.setHeaderText(["Parachain Nodes", userProject?.projectName]);
      setStatus(NETWORK_STATE.SUCCESS); */
    } catch (err) {
      //console.log(err);
      if (err.response.status === 401) {
        alertError(err.response.data.message);
        auth.logout();
      }
      setStatus(NETWORK_STATE.ERROR);
    }
  };

  useEffect(() => {
    fetchProject();
  }, []);

  useEffect(() => {
    reloadParachainList && navigate(appConfig.nav.routes.nodes);
  }, [reloadParachainList]);

  const handleCopy = (id, text) => {
    navigator.clipboard.writeText(text);
    setIsCopied({ copy: true, id: id });
    setTimeout(() => {
      setIsCopied({ copy: false, id: "" });
    }, 2000);
  };

  return (
    <div className="parachain-project">
      <div className="parachain-project__header">
        <h2>
          {activeNodeType &&
            capitalizeString(parachainNodes[activeNodeType][0]?.projectName)}
        </h2>
      </div>
      <div className="w-full min-h-[75vh] bg-white rounded-[10px] border border-[#D0D2D5] mt-8">
        {activeNodeType && (
          <div className="w-full">
            <div className="parachain-project__header bg-[#F4F6F9] h-[105px] rounded-t-[10px] px-8">
              <ProjectIcon width={56} height={56} />
              <h2>
                {capitalizeString(
                  parachainNodes[activeNodeType][0]?.projectName
                )}
              </h2>
            </div>
            <div className="w-full px-8">
              <div className="w-full border-b border-b-[#D0D2D5] mt-8 text-14">
                {selectionTabs.map((tab) => {
                  return (
                    <button
                      type="button"
                      className={`${
                        activeTab === tab
                          ? "text-black border-b-[2px] border-b-blue"
                          : "text-[#686A6D]"
                      } pb-2 w-20 -mb-[1px]`}
                      onClick={() => setActiveTab(tab)}
                    >
                      {tab}
                    </button>
                  );
                })}
              </div>

              {activeTab === "Overview" && (
                <div className="w-full border border-[#D0D2D5] mt-8 rounded-[10px]">
                  <table className="overview-table">
                    <tr>
                      <th>Network</th>
                      <th>Cloud Provider</th>
                      <th>Status</th>
                      <th>Date Created</th>
                    </tr>
                    <tr>
                      <td>
                        <div className="flex items-center gap-1">
                          <div className="w-6 h-6">
                            {
                              networkIcons[
                                capitalizeString(
                                  parachainNodes[activeNodeType][0]?.networkName
                                )
                              ]
                            }
                          </div>
                          {capitalizeString(
                            parachainNodes[activeNodeType][0]?.networkName
                          )}
                        </div>
                      </td>
                      <td>
                        <div className="flex items-center gap-1">
                          {cloudProvider && (
                            <div className="rounded-full bg-[#EBF1FF] w-6 h-6 flex items-center justify-center">
                              <img
                                src={cloudProvider.icon}
                                alt=""
                                width={17}
                                className="ml-[3px]"
                              />
                            </div>
                          )}
                          {
                            appConfig.cloudProviders.filter(
                              (provider) =>
                                provider.slug ===
                                parachainNodes[activeNodeType][0]?.cloudProvider
                            )[0]?.label
                          }
                        </div>
                      </td>
                      <td>
                        <StatusDiv
                          status={parachainNodes[activeNodeType][0]?.status}
                        />
                      </td>
                      <td>
                        <div className="flex items-center gap-1">
                          <DateIcon />
                          {
                            getDateTime(
                              parachainNodes[activeNodeType][0]?.createdAt
                            ).date
                          }
                        </div>
                      </td>
                    </tr>
                  </table>
                </div>
              )}

              {activeTab === "Graph" && (
                <div className="flex-grow mt-8">
                  <div className="flex gap-5 text-14 mb-4">
                    {Object.keys(parachainNodes).map((nodeType) => {
                      return (
                        <button
                          type="button"
                          className="flex gap-2 items-center"
                          onClick={() => setActiveNodeType(nodeType)}
                        >
                          <input
                            type="radio"
                            checked={activeNodeType === nodeType}
                            onChange={() => {}}
                            className="w-4 h-4"
                          />
                          <p>{capitalizeString(nodeType)}</p>
                        </button>
                      );
                    })}
                  </div>

                  {parachainNodes[activeNodeType] ? (
                    <ParachainCPUChart
                      nodes={parachainNodes}
                      activeNodeType={activeNodeType}
                    />
                  ) : (
                    <img
                      src={Loader}
                      alt="Loading..."
                      className="self-center py-20"
                    />
                  )}
                </div>
              )}

              {activeTab === "Nodes" && (
                <div className="w-full mt-8">
                  <div className="flex gap-5 text-14 mb-4">
                    {Object.keys(parachainNodes).map((nodeType) => {
                      return (
                        <button
                          type="button"
                          className="flex gap-2 items-center"
                          onClick={() => setActiveNodeType(nodeType)}
                        >
                          <input
                            type="radio"
                            checked={activeNodeType === nodeType}
                            onChange={() => {}}
                            className="w-4 h-4"
                          />
                          <p>{capitalizeString(nodeType)}</p>
                        </button>
                      );
                    })}
                  </div>

                  <div className="w-[245px] h-[65px] bg-[#F6F9FF] rounded-[4px] py-3 px-5 mt-6 flex justify-between items-center">
                    <div className="flex flex-col gap-1">
                      <p className="text-[12px] text-[#686A6D]">Storage size</p>
                      <p className="text-14 font-[500]">
                        {convertToTeraByte(
                          parachainNodes[activeNodeType][0]?.storageSize,
                          "gigabyte"
                        )}
                      </p>
                    </div>
                    <div className="flex flex-col gap-1">
                      <p className="text-[12px] text-[#686A6D]">Memory</p>
                      <p className="text-14 font-[500]">
                        {parachainNodes[activeNodeType][0]?.memorySize}GB
                      </p>
                    </div>
                    <div className="flex flex-col gap-1">
                      <p className="text-[12px] text-[#686A6D]">CPU</p>
                      <p className="text-14 font-[500]">
                        {getUnit(
                          parachainNodes[activeNodeType][0]?.cpuCores,
                          "core"
                        )}
                      </p>
                    </div>
                  </div>
                </div>
              )}

              {activeTab === "Endpoints" &&
                [parachainNodes[activeNodeType][0]].map((node, index) => {
                  return (
                    <div key={index} className="dedicatedProject__rpcApi">
                      <div className="dedicatedProject__insight h-[80px]">
                        <div className="dedicatedProject__insight-title flex gap-2 items-center">
                          RPC Endpoint
                        </div>
                        <div className="dedicatedProject__rpcApi-value">
                          <div className="w-full flex justify-between items-center">
                            <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                              {`https://${
                                node?.referenceId
                              }-node.${"blockops.network"}`}
                            </p>
                            <div
                              className="p-2 cursor-pointer hover:scale-[1.05]"
                              onClick={() =>
                                handleCopy(
                                  `RPC-${node.referenceId}`,
                                  `https://${
                                    node?.referenceId
                                  }-node.${"blockops.network"}`
                                )
                              }
                            >
                              <Copy />
                              {isCopied.id === `RPC-${node.referenceId}` && (
                                <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                                  Copied!
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                      <div className="dedicatedProject__insight h-[80px]">
                        <div className="dedicatedProject__insight-title flex gap-2 items-center">
                          API Key
                        </div>
                        <div className="dedicatedProject__rpcApi-value">
                          <div className="w-full flex justify-between items-center">
                            <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                              {node?.apiKey?.slice(0, 10)}
                              ********
                            </p>
                            <div
                              className="p-2 cursor-pointer hover:scale-[1.05]"
                              onClick={() =>
                                handleCopy(
                                  `API-${node.referenceId}`,
                                  `${node?.apiKey}`
                                )
                              }
                            >
                              <Copy />
                              {isCopied.id === `API-${node.referenceId}` && (
                                <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                                  Copied!
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                })}
            </div>
          </div>
        )}

        {/* <div className="flex gap-[70px] mt-10 p-8">
          <div className="flex flex-col gap-5">
            {selectionTabs.map((tab, idx) => {
              return (
                <button
                  key={idx}
                  type="button"
                  className={`text-14 ${activeTab === tab && "text-blue"}`}
                  onClick={() => setActiveTab(tab)}
                >
                  {tab}
                </button>
              );
            })}
          </div>

          {activeTab === "Graph" && (
            <div className="w-ful flex-grow">
              {parachainNodes[activeNodeType] ? (
                <ParachainCPUChart
                  nodes={parachainNodes}
                  activeNodeType={activeNodeType}
                />
              ) : (
                <img
                  src={Loader}
                  alt="Loading..."
                  className="self-center py-20"
                />
              )}
            </div>
          )}

          {activeTab === "Nodes" && (
            <div className="w-full">
              <div className="w-full h-[45px] flex items-end text-14 text-black font-[500] border-b border-b-[#686A6D]">
                {Object.keys(parachainNodes)?.map((key, index) => {
                  const node = parachainNodes[key][0];
                  return (
                    <button
                      key={index}
                      type="button"
                      className={`w-[110px] h-[45px] rounded-t-[2px] flex items-center justify-center pl-2 ${
                        activeNodeType === node.nodeType
                          ? "bg-[#F4F6F9] border-l border-r border-t border-[#686A6D]"
                          : "hover:bg-[#F4F6F9]"
                      }`}
                      onClick={() => {
                        setActiveNodeType(node.nodeType);
                      }}
                    >
                      <p className="mr-1 truncate">
                        {capitalizeString(node.nodeType)}
                      </p>
                    </button>
                  );
                })}
              </div>

              <div className="w-[245px] h-[65px] bg-[#F6F9FF] rounded-[4px] py-3 px-5 mt-6 flex justify-between items-center">
                <div className="flex flex-col gap-1">
                  <p className="text-[12px] text-[#686A6D]">Storage size</p>
                  <p className="text-14 font-[500]">
                    {convertToTeraByte(
                      parachainNodes[activeNodeType][0]?.storageSize,
                      "gigabyte"
                    )}
                  </p>
                </div>
                <div className="flex flex-col gap-1">
                  <p className="text-[12px] text-[#686A6D]">Memory</p>
                  <p className="text-14 font-[500]">
                    {parachainNodes[activeNodeType][0]?.memorySize}GB
                  </p>
                </div>
                <div className="flex flex-col gap-1">
                  <p className="text-[12px] text-[#686A6D]">CPU</p>
                  <p className="text-14 font-[500]">
                    {getUnit(
                      parachainNodes[activeNodeType][0]?.cpuCores,
                      "core"
                    )}
                  </p>
                </div>
              </div>

              {activeNodeType &&
                [parachainNodes[activeNodeType][0]].map((node, index) => {
                  return (
                    <div key={index} className="dedicatedProject__rpcApi">
                      <div className="dedicatedProject__insight h-[80px]">
                        <div className="dedicatedProject__insight-title flex gap-2 items-center">
                          RPC Endpoint
                        </div>
                        <div className="dedicatedProject__rpcApi-value">
                          <div className="w-full flex justify-between items-center">
                            <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                              {`https://${
                                node?.referenceId
                              }-node.${"blockops.network"}`}
                            </p>
                            <div
                              className="p-2 cursor-pointer hover:scale-[1.05]"
                              onClick={() =>
                                handleCopy(
                                  `RPC-${node.referenceId}`,
                                  `https://${
                                    node?.referenceId
                                  }-node.${"blockops.network"}`
                                )
                              }
                            >
                              <Copy />
                              {isCopied.id === `RPC-${node.referenceId}` && (
                                <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                                  Copied!
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                      <div className="dedicatedProject__insight h-[80px]">
                        <div className="dedicatedProject__insight-title flex gap-2 items-center">
                          API Key
                        </div>
                        <div className="dedicatedProject__rpcApi-value">
                          <div className="w-full flex justify-between items-center">
                            <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                              {node?.apiKey?.slice(0, 10)}
                              ********
                            </p>
                            <div
                              className="p-2 cursor-pointer hover:scale-[1.05]"
                              onClick={() =>
                                handleCopy(
                                  `API-${node.referenceId}`,
                                  `${node?.apiKey}`
                                )
                              }
                            >
                              <Copy />
                              {isCopied.id === `API-${node.referenceId}` && (
                                <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                                  Copied!
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                })}
            </div>
          )}
        </div> */}
      </div>

      <TerminateModal
        terminateModalIsOpen={terminateModalIsOpen}
        setTerminateModalIsOpen={setTerminateModalIsOpen}
        project={activeNodeType && parachainNodes[activeNodeType][0]}
        updateProjectStatus={setLiveProjectStatus}
        projectType={"parachain"}
      />
    </div>
  );
};

export default ParachainProjectInsight;

/* return (
  <div className="w-full">
    {status === NETWORK_STATE.LOADING ? (
      <img
        src={Loader}
        alt="Loading..."
        className="self-center mx-auto py-20"
      />
    ) : status === NETWORK_STATE.SUCCESS && parachainNodes ? (
      <div className="dedicatedProject">
        <div className="dedicatedProject__header">
          <button
            className="create__back"
            onClick={() => {
              navigate(appConfig.nav.routes.parachain);
            }}
          >
            <Back className="scale-[0.9]" />
            <p>Go Back</p>
          </button>
          <div className="flex items-center gap-3">
            <button
              className={`dedicatedProject__header-btn ${
                liveProjectStatus === "terminate" &&
                "cursor-not-allowed hover:opacity-70"
              }`}
              onClick={() => setTerminateModalIsOpen(!terminateModalIsOpen)}
              disabled={liveProjectStatus === "terminate"}
            >
              <Bin />
            </button>
            <div className="w-fit bg-[#F1F6FF bg-white p-3 flex items-center rounded-[8px] gap-5 self-center mx-auto">
              {Object.keys(parachainNodes).map((key, index) => {
                return (
                  <button
                    key={index}
                    className={`py-2 px-3 rounded-[8px] text-[16px] ${
                      activeNodeType === key
                        ? "bg-blue text-white"
                        : "bg-transparent"
                    }`}
                    type="button"
                    onClick={() => setActiveNodeType(key)}
                  >
                    {key === "rpc"
                      ? `RPC`
                      : `${capitalizeString(key)} Nodes (${
                          parachainNodes[key][0]?.count
                        })`}
                  </button>
                );
              })}
            </div>
          </div>
        </div>

        <div className="project-insight__graph mt-5">
          <div className="w-full flex items-center justify-between">
          </div>
          {parachainNodes[activeNodeType] ? (
            <ParachainCPUChart
              nodes={parachainNodes}
              activeNodeType={activeNodeType}
            />
          ) : (
            <img
              src={Loader}
              alt="Loading..."
              className="self-center py-20"
            />
          )}
        </div>

        <div className="dedicatedProject__dataBox">
          <div className="dedicatedProject__insight h-[120px]">
            <p className="dedicatedProject__insight-title">Storage size</p>
            <p className="dedicatedProject__insight-value">
              {parachainNodes[activeNodeType][0]?.storageSize}
            </p>
          </div>
          <div className="dedicatedProject__insight h-[120px]">
            <p className="dedicatedProject__insight-title">Memory GB</p>
            <p className="dedicatedProject__insight-value">
              {parachainNodes[activeNodeType][0]?.memorySize}
            </p>
          </div>
          <div className="dedicatedProject__insight h-[120px]">
            <p className="dedicatedProject__insight-title">Uptime</p>
            <p className="dedicatedProject__insight-value">
              {liveProjectStatus === "active" ? (
                <LiveUptime
                  startDate={parachainNodes[activeNodeType][0]?.createdAt}
                />
              ) : (
                `${capitalizeString(liveProjectStatus)}d`
              )}
            </p>
          </div>
        </div>

        {[parachainNodes[activeNodeType][0]].map((node, index) => {
          return (
            <div key={index} className="dedicatedProject__rpcApi">
              <div className="dedicatedProject__insight h-[80px]">
                <div className="dedicatedProject__insight-title flex gap-2 items-center">
                  RPC Endpoint
                </div>
                <div className="dedicatedProject__rpcApi-value">
                  <div className="w-full flex justify-between items-center">
                    <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                      {`https://${
                        node?.referenceId
                      }-node.${"blockops.network"}`}
                    </p>
                    <div
                      className="p-2 cursor-pointer hover:scale-[1.05]"
                      onClick={() =>
                        handleCopy(
                          `RPC-${node.referenceId}`,
                          `https://${
                            node?.referenceId
                          }-node.${"blockops.network"}`
                        )
                      }
                    >
                      <Copy />
                      {isCopied.id === `RPC-${node.referenceId}` && (
                        <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                          Copied!
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
              <div className="dedicatedProject__insight h-[80px]">
                <div className="dedicatedProject__insight-title flex gap-2 items-center">
                  API Key
                </div>
                <div className="dedicatedProject__rpcApi-value">
                  <div className="w-full flex justify-between items-center">
                    <p className="w-full font-[200] text-[14px] text-ellipsis truncate">
                      {node?.apiKey?.slice(0, 10)}
                      ********
                    </p>
                    <div
                      className="p-2 cursor-pointer hover:scale-[1.05]"
                      onClick={() =>
                        handleCopy(
                          `API-${node.referenceId}`,
                          `${node?.apiKey}`
                        )
                      }
                    >
                      <Copy />
                      {isCopied.id === `API-${node.referenceId}` && (
                        <div className="absolute bg-[#6E6E6F] text-white text-[12px] font-bol p-1 rounded-lg">
                          Copied!
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          );
        })}
        <div>
          <TerminateModal
            terminateModalIsOpen={terminateModalIsOpen}
            setTerminateModalIsOpen={setTerminateModalIsOpen}
            project={parachainNodes[activeNodeType][0]}
            updateProjectStatus={setLiveProjectStatus}
            projectType={"parachain"}
          />
        </div>
      </div>
    ) : (
      <div className="mt-5 text-[14px]">
        <p>An Error has occured</p>
        <p
          className="underline text-blue cursor-pointer"
          onClick={fetchProject}
        >
          Retry
        </p>
      </div>
    )}
  </div>
); */

/*
const response = [
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "boot",
      count: 2,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 8,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "3892087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "collator",
      count: 1,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 6,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "7362087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
    {
      parachainId: 1,
      organizationId: 6,
      createdBy: 6,
      networkName: "kusama",
      projectName: "eewee",
      nodeType: "validator",
      count: 5,
      imageVersion: "Polkadot-v0.9.31",
      cloudProvider: "gcp",
      cpuCores: 8,
      storageSize: 1000,
      memorySize: 16,
      ami: "Ubuntu",
      nodeKey: "System Generated",
      argumentRules:
        " --rpc-cors=all  --ws-external=true  --rpc-external=true  --rpc-methods=Unsafe  --name=eewee   ",
      referenceId: "7362087453",
      apiKey:
        "6ec717d353111fa062aa05d26b89def0:c243ed8e8a13e99c14a2115af18096db189eef0af90bbe994cdb0ffbe2e249883a05b0a65960fca888e0a7c7ed37716834f7c14864ebe674efb2085e3a486a63:c6b6727038ada457fb399dea9b500aac",
      status: "active",
      createdAt: "2023-08-23T11:21:09.000Z",
      updatedAt: "2023-08-23T11:21:09.000Z",
      organization: {
        id: 6,
      },
    },
  ]; 
 */
