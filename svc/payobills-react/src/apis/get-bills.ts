import { BillProp } from "../components/Bill"

export const getBills = async ({ limit, offset }: { limit: number, offset: number }, cb: any) => {
    try {
        var response = await fetch(`${import.meta.env.VITE_NOCODB_BASE_URL}/payobills/bills?limit=${limit}&offset=${offset}`, {
            headers: {
                'xc-token': import.meta.env.VITE_NOCODB_API_TOKEN
            }
        })

        var responseJson = await response.json()
        var bills: BillProp[] = responseJson.list.map((bill: any) => ({
            id: bill.Id,
            name: bill.Name
        }))

        cb(null, bills)
        // cb(null, bills)
    }
    catch (err) {
        cb('Uh oh, something went wrong. Unable to retrieve your bills.', [])
    }
}
