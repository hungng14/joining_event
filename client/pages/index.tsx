import type { NextPage } from "next";
import styles from "../styles/Home.module.css";
import * as NearApi from "near-api-js";
import { useEffect, useState } from "react";
console.log("nearApi", NearApi);

const Home: NextPage = () => {
  const [NearProvider, setNearProvider] = useState<NearApi.Near | null>(null);
  const [NearWallet, setNearWallet] = useState<NearApi.WalletConnection | null>(
    null
  );
  const [JoiningEventContract, setJoiningEventContract] = useState<any>();

  useEffect(() => {
    const keyStore = new NearApi.keyStores.BrowserLocalStorageKeyStore();

    const config = {
      networkId: "testnet",
      keyStore: keyStore,
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org",
    };
    const connectToNear = async () => {
      const near = await NearApi.connect(config);
      setNearProvider(near);
      const wallet = new NearApi.WalletConnection(near, null);
      setNearWallet(wallet);
    };
    connectToNear();
  }, []);

  useEffect(() => {
    const initContract = () => {
      if (!NearWallet) return;
      const contractName = "joining_event.nurota.testnet";
      const methodOptions = {
        viewMethods: ["get_member"],
        changeMethods: ["register"],
      };
      const contract: any = new NearApi.Contract(
        NearWallet.account(),
        contractName,
        methodOptions
      );
      setJoiningEventContract(contract);
    };
    initContract();
  }, [NearWallet]);

  const onConnectWallet = () => {
    if (!NearWallet) return;
    NearWallet.requestSignIn({ contractId: "" });
  };

  const onDisconnectWallet = () => {
    if (!NearWallet) return;
    NearWallet.signOut();
    if(!NearProvider) return setNearWallet(null);
    const wallet = new NearApi.WalletConnection(NearProvider, null);
    setNearWallet(wallet);
  };

  const onRegister = async () => {
    const result = await JoiningEventContract.register({
      email: "abc@gmail.com",
    });
    console.log("result", result);
    if (result?.success) {
      return alert("Register successfully");
    }
    return alert(result?.message || "Register failed");
  };

  const onGetInfor = async () => {
    const myAccountId = NearWallet?.getAccountId();
    console.log("myAccountId", myAccountId);
    const info = await JoiningEventContract.get_member({
      account_id: myAccountId,
    });
    console.log("info", info);
  };

  return (
    <div
      className={styles.container}
      style={{
        display: "flex",
        gap: "10px",
        justifyContent: "center",
        marginTop: "50px",
      }}
    >
      {!NearWallet?.isSignedIn() ? (
        <button
          onClick={onConnectWallet}
          style={{ background: "orange", color: "#000", padding: "8px 16px" }}
        >
          Connect Wallet
        </button>
      ) : (
        <button
          onClick={onDisconnectWallet}
          style={{ background: "orange", color: "#000", padding: "8px 16px" }}
        >
          Disconnect Wallet
        </button>
      )}

      <button
        onClick={onRegister}
        style={{ background: "pink", color: "#000", padding: "8px 16px" }}
      >
        Register
      </button>
      <button
        onClick={onGetInfor}
        style={{ background: "#007788", color: "#000", padding: "8px 16px" }}
      >
        Get Infor
      </button>
    </div>
  );
};

export default Home;
