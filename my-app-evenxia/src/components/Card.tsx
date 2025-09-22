type CardProps = {
    readonly title?: string;
    readonly description?: string;
    readonly imageUrl?: string;
    readonly start_date?: string;
    readonly end_date?: string;
    readonly address?: string;
    readonly onButtonClick?: () => void;
};

export default function Card({ title = "", description = "", imageUrl = "", address = "", start_date = "", end_date = "", onButtonClick }: CardProps) {
    return (
        <div className="card bg-base-100  hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105">
            <figure className="m-4 rounded-[2rem]">
                <img
                    srcSet={imageUrl}
                    className=""
                    alt="Event" />
            </figure>
            <div className="card-body flex flex-col justify-between">
                <div className="max-sm:flex flex-col justify-center items-center">
                    <h2 className="card-title">{title}</h2>
                    <div className="flex flex-col gap-2 justify-around">
                        <p>{description}</p>
                    </div>
                </div>
                <div className="card-actions justify-end max-sm:justify-none">
                    <div className="flex flex-row justify-between items-center w-full max-sm:flex-col gap-2">
                        <p className="font-bold">{start_date} - {end_date}</p>
                        <button onClick={onButtonClick} className="btn btn-primary max-sm:btn-sm p-4">Voir plus en détails</button>
                    </div>
                    <div className="max-sm:text-sm">
                        <p className="italic">{address}</p>
                    </div>
                </div>
            </div>
        </div>)
}