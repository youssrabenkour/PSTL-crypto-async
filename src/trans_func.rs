

const MASK_L : [u64; 6] = [
	0xaaaaaaaaaaaaaaaa,
	0xcccccccccccccccc,
	0xf0f0f0f0f0f0f0f0,
	0xff00ff00ff00ff00,
	0xffff0000ffff0000,
	0xffffffff00000000
];

const  MASK_R : [u64; 6] = [
	0x5555555555555555,
	0x3333333333333333,
	0x0f0f0f0f0f0f0f0f,
	0x00ff00ff00ff00ff,
	0x0000ffff0000ffff,
	0x00000000ffffffff	
];

//la fonction real_ortho mut = varie le tableau a 64 element et chaque ele est code sur 64 bit

fn real_ortho(data: & mut [u64;64]) 
{
	for i in 0..6 {
	
		let n = (1u64 << i) as usize; //usize la taille
	
		//je vais faire while mieux que for 
	
		let mut j : usize = 0; //j change de valeur j est de type de n

		while j<64 {
		
		 	for k in 0..n {

			let u : u64 = data[j + k] & MASK_L[i];
			let v : u64 = data[j + k] & MASK_R[i];
			let x : u64 = data[j + n + k] & MASK_L[i];
			let y : u64 = data[j + n + k] & MASK_R[i];
			data[j + k] = u | (x >> n);
			data[j + n + k] = (v << n) | y;

			}
	
		j += 2 * n;
	
		}

	}
}


fn main() {
    	
	
	let mut data :[u64; 64] = [0x0000000000000001 ;64]; 
	
	for i in 0..64 {
  		println!("{:?}",data[i]);
  	}

	real_ortho(&mut data);

	//apres transposer
	for i in 0..64 {
  		println!("{:?}",data[i]);
  	}


}
