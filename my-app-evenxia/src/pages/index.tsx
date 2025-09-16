
import Button from "@/components/Button";
import Card from "@/components/Card";


export default function Home() {
  return (
    <div>
      <div className="flex flex-col gap-4 p-4">
        <div className="navbar bg-base-100 rounded-full  flex justify-between">
          <div className="m-4 font-bold">Evenxia</div>
          <div className="m-4 flex gap-2">
            <button className="btn">Login</button>
            <button className="btn">Sign up</button>
          </div>
        </div>
        <div className="flex items-center justify-center gap-4">
          <input type="text" placeholder="Chercher un événement ..." className="input input-neutral " />
        </div>
        <div className="grid grid-cols-4 gap-4">
          <Card></Card>
          <Card></Card>
          <Card></Card>
          <Card></Card>
          <Card></Card>
        </div>
      </div>
    </div>
  );
}
