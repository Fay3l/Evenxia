type CardProps = {
    readonly title?: string;
    readonly description?: string;
    readonly imageUrl?: string;
    readonly start_date?: string;
    readonly end_date?: string;
    readonly address?: string;
    readonly views?: { views: number};
    readonly onButtonClick?: () => void;
};

export default function Card({ title = "", description = "", imageUrl = "", address = "", start_date = "", end_date = "", views={views: 0}, onButtonClick }: CardProps) {
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
                    <div className="flex flex-row justify-between items-center w-full max-sm:flex-col gap-2">
                        <div className="flex flex-row justify-center items-center gap-1 max-sm:text-sm">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                                <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                            </svg>
                            <p>{views.views}</p>
                        </div>
                        <div className="max-sm:text-sm">
                            <p className="italic">{address}</p>
                        </div>
                    </div>

                </div>
            </div>
        </div>)
}