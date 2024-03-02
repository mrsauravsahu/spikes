import type { Bill as BillModel } from "../models/Bill"

export type BillProp = BillModel

export const Bill = ({ bill } : { bill: BillProp }) => {
    return (
        <div>
            <div>{bill.name}</div>
        </div>
    )
}
