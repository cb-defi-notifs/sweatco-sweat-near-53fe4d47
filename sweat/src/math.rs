pub fn formula(mut steps_from_tge: f64, mut steps: f64) -> u128 {
    let mut tokens: u128 = 0;
    let first_trillion = (steps_from_tge / 1e+12).floor() as usize;
    let last_trillion = ((steps_from_tge + steps as f64) / 1e+12).floor() as usize;
    for trillion in first_trillion..last_trillion + 1 {
        let steps_for_current_line = f64::min(steps, (trillion as f64 + 1.) * 1e+12 - steps_from_tge);
        if trillion < 400 {
            tokens += (area_under_line(
                KS[trillion],
                BS[trillion],
                steps_from_tge,
                steps_from_tge + steps_for_current_line,
            ) * 1e+18) as u128
        } else {
            tokens += (exp_decay(steps_from_tge, steps_for_current_line) * 1e+18) as u128;
        }
        steps_from_tge += steps_for_current_line;
        steps -= steps_for_current_line;
    }
    return tokens;
}
pub fn area_under_line(k: f64, b: f64, x_start: f64, x_end: f64) -> f64 {
    let square_area = (k * x_end + b) * (x_end - x_start);
    let triangle_area = ((k * x_start + b) - (k * x_end + b)) * (x_end - x_start) / 2.;
    return square_area + triangle_area;
}
pub fn exp_decay(steps_from_tge: f64, steps_to_exchange: f64) -> f64 {
    const K: f64 = 0.00000000021;
    return ((K * (steps_from_tge + steps_to_exchange) + 1000.).ln() - (K * steps_from_tge + 1000.).ln()) / K;
}

// one line per trillion, y = KS[i] * x + BS[i]
pub const KS: [f64; 400] = [
    -1.742520661157025e-16,
    -1.2211765219415668e-16,
    -9.065175408277882e-17,
    -6.995950586823161e-17,
    -5.562633881230116e-17,
    -4.5288700625944296e-17,
    -3.758778796890118e-17,
    -3.1697164481237514e-17,
    -2.709065614832411e-17,
    -2.3420309186293122e-17,
    -2.0448547899814824e-17,
    -1.8008664343586945e-17,
    -1.598087908237869e-17,
    -1.42773335964399e-17,
    -1.2832398630053257e-17,
    -1.1596250138167316e-17,
    -1.0530511613434258e-17,
    -9.60523653442959e-18,
    -8.796779751972532e-18,
    -8.086270618159395e-18,
    -7.458500995307857e-18,
    -6.901104123772342e-18,
    -6.403940533380575e-18,
    -5.958633410198423e-18,
    -5.5582132450331125e-18,
    -5.1968433436532485e-18,
    -4.869605831759065e-18,
    -4.5723333827272365e-18,
    -4.301475833962034e-18,
    -4.053993662692963e-18,
    -3.827272312715457e-18,
    -3.619052834562552e-18,
    -3.4273753830472574e-18,
    -3.2505329185657195e-18,
    -3.0870330591887344e-18,
    -2.9355664838547095e-18,
    -2.794980631720291e-18,
    -2.66425770685141e-18,
    -2.5424962012064024e-18,
    -2.428895307109955e-18,
    -2.322741714083282e-18,
    -2.2233983821163942e-18,
    -2.1302949603328596e-18,
    -2.0429195811004406e-18,
    -1.960811808462916e-18,
    -1.883556558973762e-18,
    -1.8107788446436025e-18,
    -1.7421392133484344e-18,
    -1.677329782913845e-18,
    -1.6160707821465619e-18,
    -1.558107526083253e-18,
    -1.5032077642581656e-18,
    -1.4511593503267131e-18,
    -1.4017681892945333e-18,
    -1.3548564251906402e-18,
    -1.3102608375276288e-18,
    -1.2678314195043969e-18,
    -1.2274301147857137e-18,
    -1.1889296929627269e-18,
    -1.1522127465624104e-18,
    -1.1171707948204621e-18,
    -1.0837034814235483e-18,
    -1.0517178551271487e-18,
    -1.0211277236068263e-18,
    -9.918530721450737e-19,
    -9.638195398233447e-19,
    -9.36957946809014e-19,
    -9.112038671192228e-19,
    -8.864972419294395e-19,
    -8.627820290880181e-19,
    -8.400058850140189e-19,
    -8.181198756030062e-19,
    -7.970782131563826e-19,
    -7.768380166916919e-19,
    -7.573590932880974e-19,
    -7.386037383835583e-19,
    -7.205365531697463e-19,
    -7.031242774314684e-19,
    -6.863356363557849e-19,
    -6.701411999923302e-19,
    -6.545132541846272e-19,
    -6.394256819147309e-19,
    -6.248538541120077e-19,
    -6.107745290730079e-19,
    -5.971657597249509e-19,
    -5.840068080416249e-19,
    -5.712780659877686e-19,
    -5.589609824295272e-19,
    -5.470379955011003e-19,
    -5.354924699679231e-19,
    -5.243086391679987e-19,
    -5.134715511536882e-19,
    -5.029670186897631e-19,
    -4.927815727953693e-19,
    -4.829024195460038e-19,
    -4.733173998763812e-19,
    -4.640149521483529e-19,
    -4.5498407726872845e-19,
    -4.4621430616028325e-19,
    -4.376956694063203e-19,
    -4.294186689042585e-19,
    -4.213742513779501e-19,
    -4.1355378361045103e-19,
    -4.059490292708393e-19,
    -3.985521272190651e-19,
    -3.9135557118156494e-19,
    -3.843521907002706e-19,
    -3.7753513326384618e-19,
    -3.7089784753883453e-19,
    -3.6443406762322e-19,
    -3.5813779825251673e-19,
    -3.5200330089256506e-19,
    -3.4602508065903914e-19,
    -3.401978740080183e-19,
    -3.3451663714584556e-19,
    -3.289765351110967e-19,
    -3.2357293148394285e-19,
    -3.1830137868240586e-19,
    -3.131576088075919e-19,
    -3.0813752500226847e-19,
    -3.032371932907725e-19,
    -2.9845283486919426e-19,
    -2.937808188183263e-19,
    -2.892176552124129e-19,
    -2.847599885999716e-19,
    -2.804045918333272e-19,
    -2.761483602261653e-19,
    -2.719883060190318e-19,
    -2.6792155313445137e-19,
    -2.63945332204532e-19,
    -2.6005697585498377e-19,
    -2.5625391423035217e-19,
    -2.525336707468934e-19,
    -2.488938580593963e-19,
    -2.4533217423026255e-19,
    -2.4184639908895905e-19,
    -2.384343907712918e-19,
    -2.350940824282565e-19,
    -2.3182347909537016e-19,
    -2.286206547131352e-19,
    -2.254837492907322e-19,
    -2.2241096620501493e-19,
    -2.1940056962730563e-19,
    -2.1645088207149877e-19,
    -2.135602820565552e-19,
    -2.1072720187766023e-19,
    -2.0795012548015057e-19,
    -2.0522758643085016e-19,
    -2.0255816598175961e-19,
    -1.9994049122138986e-19,
    -1.9737323330903044e-19,
    -1.9485510578807662e-19,
    -1.9238486297398327e-19,
    -1.8996129841342056e-19,
    -1.8758324341093465e-19,
    -1.8524956561967793e-19,
    -1.829591676932681e-19,
    -1.8071098599545532e-19,
    -1.785039893650975e-19,
    -1.763371779334955e-19,
    -1.7420958199175433e-19,
    -1.7212026090561526e-19,
    -1.7006830207563481e-19,
    -1.680528199404236e-19,
    -1.6607295502106764e-19,
    -1.641278730046456e-19,
    -1.6221676386514405e-19,
    -1.6033884101995837e-19,
    -1.584933405204139e-19,
    -1.5667952027467749e-19,
    -1.548966593016198e-19,
    -1.5314405701426605e-19,
    -1.514210325313819e-19,
    -1.4972692401612018e-19,
    -1.4806108804030585e-19,
    -1.464228989734711e-19,
    -1.4481174839530237e-19,
    -1.4322704453063565e-19,
    -1.4166821170598995e-19,
    -1.4013468982670043e-19,
    -1.386259338736829e-19,
    -1.371414134191442e-19,
    -1.3568061216030755e-19,
    -1.3424302747041397e-19,
    -1.3282816996633568e-19,
    -1.3143556309200183e-19,
    -1.3006474271706087e-19,
    -1.2871525675013874e-19,
    -1.2738666476598856e-19,
    -1.2607853764614205e-19,
    -1.2479045723234077e-19,
    -1.235220159923208e-19,
    -1.2227281669743183e-19,
    -1.2104247211158993e-19,
    -1.1983060469117014e-19,
    -1.186368462952876e-19,
    -1.1746083790623575e-19,
    -1.1630222935948628e-19,
    -1.1516067908299945e-19,
    -1.140358538454452e-19,
    -1.12927428512993e-19,
    -1.118350858143447e-19,
    -1.1075851611362834e-19,
    -1.0969741719101296e-19,
    -1.0865149403050667e-19,
    -1.0762045861494076e-19,
    -1.0660402972764542e-19,
    -1.0560193276064079e-19,
    -1.0461389952919775e-19,
    -1.0363966809234148e-19,
    -1.026789825792334e-19,
    -1.0173159302116731e-19,
    -1.007972551889153e-19,
    -9.987573043534226e-20,
    -9.89667855429671e-20,
    -9.807019257644345e-20,
    -9.71857287395974e-20,
    -9.631317623701567e-20,
    -9.545232213998049e-20,
    -9.460295825653477e-20,
    -9.376488100558262e-20,
    -9.293789129490628e-20,
    -9.212179440291706e-20,
    -9.131639986400428e-20,
    -9.05215213574181e-20,
    -8.97369765994728e-20,
    -8.89625872390465e-20,
    -8.819817875622532e-20,
    -8.744358036393566e-20,
    -8.66986249125548e-20,
    -8.596314879736066e-20,
    -8.523699186866498e-20,
    -8.451999734463336e-20,
    -8.381201172666998e-20,
    -8.311288471723175e-20,
    -8.242246914006826e-20,
    -8.17406208627657e-20,
    -8.106719872148295e-20,
    -8.040206444793726e-20,
    -7.974508259835858e-20,
    -7.909612048459869e-20,
    -7.845504810707319e-20,
    -7.782173808967559e-20,
    -7.719606561652087e-20,
    -7.657790837040004e-20,
    -7.596714647303045e-20,
    -7.53636624269053e-20,
    -7.47673410587931e-20,
    -7.417806946478564e-20,
    -7.35957369568366e-20,
    -7.302023501080116e-20,
    -7.245145721583091e-20,
    -7.188929922524252e-20,
    -7.133365870859258e-20,
    -7.078443530515527e-20,
    -7.024153057856528e-20,
    -6.970484797272147e-20,
    -6.91742927688726e-20,
    -6.86497720437774e-20,
    -6.813119462906056e-20,
    -6.761847107155457e-20,
    -6.711151359476315e-20,
    -6.661023606124281e-20,
    -6.61145539360551e-20,
    -6.562438425108618e-20,
    -6.513964557037604e-20,
    -6.466025795622368e-20,
    -6.418614293629506e-20,
    -6.371722347146302e-20,
    -6.325342392452462e-20,
    -6.279467002972153e-20,
    -6.234088886295495e-20,
    -6.189200881285434e-20,
    -6.14479595524933e-20,
    -6.100867201186104e-20,
    -6.057407835099107e-20,
    -6.014411193376414e-20,
    -5.971870730237195e-20,
    -5.929780015239061e-20,
    -5.888132730851492e-20,
    -5.84692267007905e-20,
    -5.806143734153396e-20,
    -5.765789930275769e-20,
    -5.725855369412674e-20,
    -5.686334264151189e-20,
    -5.647220926599343e-20,
    -5.60850976633867e-20,
    -5.570195288428931e-20,
    -5.532272091458252e-20,
    -5.4947348656356e-20,
    -5.457578390936463e-20,
    -5.420797535285113e-20,
    -5.3843872527836335e-20,
    -5.3483425819809215e-20,
    -5.3126586441820063e-20,
    -5.277330641798036e-20,
    -5.242353856733522e-20,
    -5.207723648809501e-20,
    -5.173435454225331e-20,
    -5.139484784057061e-20,
    -5.1058672227846184e-20,
    -5.072578426858286e-20,
    -5.039614123295339e-20,
    -5.006970108307852e-20,
    -4.9746422459637103e-20,
    -4.942626466875399e-20,
    -4.9109187669199714e-20,
    -4.8795152059862837e-20,
    -4.84841190675273e-20,
    -4.817605053487332e-20,
    -4.78709089088036e-20,
    -4.756865722898296e-20,
    -4.7269259116649074e-20,
    -4.697267876365857e-20,
    -4.667888092180262e-20,
    -4.638783089230199e-20,
    -4.6099494515583385e-20,
    -4.5813838161255657e-20,
    -4.5530828718280814e-20,
    -4.525043358541104e-20,
    -4.497262066177476e-20,
    -4.4697358337704964e-20,
    -4.4424615485721706e-20,
    -4.4154361451770396e-20,
    -4.388656604657526e-20,
    -4.362119953722495e-20,
    -4.335823263890888e-20,
    -4.309763650684838e-20,
    -4.283938272838032e-20,
    -4.258344331521018e-20,
    -4.232979069583107e-20,
    -4.207839770809866e-20,
    -4.182923759196701e-20,
    -4.158228398234472e-20,
    -4.1337510902137353e-20,
    -4.109489275542381e-20,
    -4.085440432073928e-20,
    -4.0616020744542986e-20,
    -4.0379717534780684e-20,
    -4.014547055460481e-20,
    -3.991325601620972e-20,
    -3.968305047478393e-20,
    -3.9454830822591013e-20,
    -3.922857428319454e-20,
    -3.9004258405740594e-20,
    -3.878186105941651e-20,
    -3.856136042796716e-20,
    -3.834273500437559e-20,
    -3.8125963585586015e-20,
    -3.7911025267386026e-20,
    -3.7697899439354905e-20,
    -3.7486565779922033e-20,
    -3.727700425153709e-20,
    -3.706919509590127e-20,
    -3.686311882932049e-20,
    -3.6658756238133044e-20,
    -3.645608837423731e-20,
    -3.625509655070073e-20,
    -3.605576233743484e-20,
    -3.585806755699572e-20,
    -3.5661994280416526e-20,
    -3.546752482316213e-20,
    -3.5274641741126006e-20,
    -3.508332782672371e-20,
    -3.489356610505922e-20,
    -3.4705339830153965e-20,
    -3.451863248125372e-20,
    -3.4333427759203314e-20,
    -3.4149709582887447e-20,
    -3.3967462085751006e-20,
    -3.378666961235e-20,
    -3.3607316715014233e-20,
    -3.3429388150533696e-20,
    -3.3252868876919535e-20,
    -3.3077744050232756e-20,
    -3.290399902145358e-20,
    -3.2731619333428746e-20,
    -3.256059071785268e-20,
    -3.239089909232153e-20,
    -3.222253055743458e-20,
    -3.205547139394996e-20,
    -3.188970805997417e-20,
    -3.172522718823971e-20,
    -3.156201558340136e-20,
    -3.140006021938326e-20,
    -3.123934823678025e-20,
    -3.1079866940326877e-20,
    -3.0921603796351267e-20,
    -3.076454643035258e-20,
    -3.0608682624554785e-20,
    -3.045400031553837e-20,
    -3.030048759191268e-20,
    -3.0148132692011986e-20,
    -2.999692400165255e-20,
    -2.9846850051913405e-20,
    -2.969789951695946e-20,
    -2.9550061211908694e-20,
    -2.9403324090731496e-20,
    -2.925767724419407e-20,
    -1.9171933427220023e-20,
];
pub const BS: [f64; 400] = [
    0.001,
    0.0009478655860784542,
    0.0008849337898556985,
    0.0008228570452120568,
    0.000765524376988335,
    0.0007138361860565507,
    0.000667630710114292,
    0.0006263963457006464,
    0.0005895442790373391,
    0.0005565111563790603,
    0.0005267935435142772,
    0.0004999548243957706,
    0.00047562140126127157,
    0.00045347530994406727,
    0.0004332462204146543,
    0.00041470399303636515,
    0.0003976521766406362,
    0.0003819225002975569,
    0.00036737027821332987,
    0.00035387060467088026,
    0.0003413152122138495,
    0.0003296098779116037,
    0.00031867227892298474,
    0.0003084302150897953,
    0.00029882013112582785,
    0.00028978588359133126,
    0.0002812777082820824,
    0.0002732513521582231,
    0.00026566734079279744,
    0.00025849035782599437,
    0.00025168871732666917,
    0.0002452339135039291,
    0.0002391002350554397,
    0.00023326443372754895,
    0.00022770543850873144,
    0.00022240410837204058,
    0.00021734301769520153,
    0.00021250626947505292,
    0.00020787933226054263,
    0.00020344889739078116,
    0.00019920275366971425,
    0.00019512967705907186,
    0.0001912193333441634,
    0.0001874621920371694,
    0.00018384945004111828,
    0.00018037296381410637,
    0.00017702518895491905,
    0.00017379912628404613,
    0.00017068827362318585,
    0.000167686582585589,
    0.00016478841978242353,
    0.00016198853192934406,
    0.00015928201440490854,
    0.000156664282870203,
    0.0001541310476085928,
    0.00015167829028712716,
    0.00014930224287782617,
    0.00014699936850886124,
    0.000144766344043128,
    0.0001426000442055093,
    0.00014049752710099242,
    0.0001384560209837807,
    0.00013647291215340392,
    0.0001345457338676236,
    0.00013267215617407142,
    0.00013084997657315903,
    0.00012907711143421321,
    0.00012735158809499723,
    0.00012567153758209196,
    0.00012403518789603387,
    0.00012244085781085392,
    0.00012088695114267202,
    0.00011937195144651513,
    0.0001178944171045927,
    0.00011645297677272671,
    0.00011504632515488627,
    0.00011367321907863656,
    0.00011233247384678917,
    0.00011102295984288586,
    0.00010974359937017294,
    0.0001084933637055567,
    0.00010727127035169509,
    0.00010607638047187179,
    0.0001049077964936348,
    0.00010376465986839802,
    0.00010264614897531531,
    0.00010155147715868368,
    0.00010047989088911667,
    9.94306680394151e-05,
    9.840311626696233e-05,
    9.739657149496913e-05,
    9.641039648566688e-05,
    9.544397949898576e-05,
    9.449673303080714e-05,
    9.356809262536679e-05,
    9.265751575675263e-05,
    9.176448077486192e-05,
    9.088848591153834e-05,
    9.002904834291072e-05,
    8.91857033042684e-05,
    8.835800325406222e-05,
    8.754551708390506e-05,
    8.674782937162016e-05,
    8.596453967464015e-05,
    8.519526186125563e-05,
    8.443962347731812e-05,
    8.369726514630092e-05,
    8.29678400006035e-05,
    8.225101314230225e-05,
    8.154646113150026e-05,
    8.08538715007229e-05,
    8.017294229376826e-05,
    7.950338162761335e-05,
    7.8844907276048e-05,
    7.819724627376032e-05,
    7.756013453976419e-05,
    7.693331651901434e-05,
    7.631654484123451e-05,
    7.570957999600647e-05,
    7.511219002317299e-05,
    7.452415021779347e-05,
    7.39452428487825e-05,
    7.337525689057661e-05,
    7.281398776704926e-05,
    7.226123710710654e-05,
    7.1716812511276e-05,
    7.118052732877359e-05,
    7.065220044446764e-05,
    7.013165607524134e-05,
    6.961872357528175e-05,
    6.911323724984047e-05,
    6.861503617701373e-05,
    6.812396403719717e-05,
    6.763986894976005e-05,
    6.716260331665613e-05,
    6.669202367258016e-05,
    6.622799054137742e-05,
    6.577036829838158e-05,
    6.531902503844327e-05,
    6.487383244931262e-05,
    6.44346656901762e-05,
    6.400140327509005e-05,
    6.357392696105534e-05,
    6.315212164057495e-05,
    6.273587523842308e-05,
    6.232507861248331e-05,
    6.19196254584469e-05,
    6.151941221819974e-05,
    6.112433799173434e-05,
    6.0734304452439244e-05,
    6.034921576558533e-05,
    5.99689785099213e-05,
    5.9593501602179115e-05,
    5.9222696224413025e-05,
    5.8856475754030186e-05,
    5.84947556963854e-05,
    5.813745361986546e-05,
    5.778448909330886e-05,
    5.743578362571232e-05,
    5.7091260608087606e-05,
    5.675084525740902e-05,
    5.641446456254062e-05,
    5.60820472320838e-05,
    5.5753523644044365e-05,
    5.542882579726999e-05,
    5.510788726456035e-05,
    5.4790643147403094e-05,
    5.447703003225709e-05,
    5.416698594833362e-05,
    5.386045032680416e-05,
    5.3557363961384356e-05,
    5.325766897024687e-05,
    5.296130875919079e-05,
    5.2668227986050514e-05,
    5.237837252625882e-05,
    5.209168943956274e-05,
    5.1808126937805044e-05,
    5.152763435375903e-05,
    5.12501621109721e-05,
    5.0975661694579276e-05,
    5.070408562303612e-05,
    5.043538742076462e-05,
    5.016952159165634e-05,
    4.9906443593405816e-05,
    4.9646109812655414e-05,
    4.938847754090365e-05,
    4.913350495116463e-05,
    4.8881151075350196e-05,
    4.8631375782329965e-05,
    4.838413975667897e-05,
    4.813940447805673e-05,
    4.7897132201212914e-05,
    4.7657285936594234e-05,
    4.7419829431526744e-05,
    4.71847271519653e-05,
    4.695194426476821e-05,
    4.6721446620514045e-05,
    4.64932007368044e-05,
    4.6267173782060014e-05,
    4.604333355978672e-05,
    4.5821648493296274e-05,
    4.5602087610867964e-05,
    4.538462053132326e-05,
    4.516921745003233e-05,
    4.4955849125289056e-05,
    4.4744486865098046e-05,
    4.45351025143152e-05,
    4.432766844214525e-05,
    4.4122157530005095e-05,
    4.3918543159702134e-05,
    4.371679920194944e-05,
    4.351690000519749e-05,
    4.3318820384760065e-05,
    4.312253561224901e-05,
    4.2928021405280725e-05,
    4.273525391747814e-05,
    4.2544209728719395e-05,
    4.235486583565916e-05,
    4.216719964250548e-05,
    4.198118895203088e-05,
    4.17968119568214e-05,
    4.161404723076193e-05,
    4.143287372074032e-05,
    4.1253270738562774e-05,
    4.107521795308747e-05,
    4.0898695382549774e-05,
    4.072368338709343e-05,
    4.055016266149302e-05,
    4.037811422805098e-05,
    4.020751942968476e-05,
    4.0038359923190114e-05,
    3.9870617672661414e-05,
    3.970427494308608e-05,
    3.953931429410061e-05,
    3.937571857389206e-05,
    3.921347091325864e-05,
    3.905255471981524e-05,
    3.889295367233122e-05,
    3.873465171522735e-05,
    3.857763305317805e-05,
    3.842188214587567e-05,
    3.826738370289203e-05,
    3.811412267868181e-05,
    3.7962084267705213e-05,
    3.781125389965173e-05,
    3.766161723479618e-05,
    3.751316015944939e-05,
    3.7365868781525676e-05,
    3.7219729426211825e-05,
    3.7074728631732516e-05,
    3.693085314522366e-05,
    3.6788089918686125e-05,
    3.664642610505785e-05,
    3.6505849054345416e-05,
    3.636634630987234e-05,
    3.622790560459189e-05,
    3.609051485749587e-05,
    3.595416217010672e-05,
    3.581883582303215e-05,
    3.5684524272620494e-05,
    3.5551216147668935e-05,
    3.541890024622637e-05,
    3.5287565532444045e-05,
    3.515720113351968e-05,
    3.5027796336687884e-05,
    3.48993405862997e-05,
    3.477182348093517e-05,
    3.464523477061422e-05,
    3.4519564354039235e-05,
    3.4394802275912814e-05,
    3.427093872431598e-05,
    3.414796402812223e-05,
    3.4025868654494865e-05,
    3.39046432064163e-05,
    3.3784278420283065e-05,
    3.366476516354382e-05,
    3.354609443238919e-05,
    3.3428257349493555e-05,
    3.3311245161798746e-05,
    3.319504923835742e-05,
    3.307966106819458e-05,
    3.29650722582435e-05,
    3.285127453130859e-05,
    3.273825972406603e-05,
    3.262601978512341e-05,
    3.251454677310065e-05,
    3.240383285475513e-05,
    3.229387030315418e-05,
    3.218465149587862e-05,
    3.207616891325115e-05,
    3.196841513662366e-05,
    3.186138284667823e-05,
    3.175506482177391e-05,
    3.1649453936321964e-05,
    3.1544543159193155e-05,
    3.144032555216044e-05,
    3.133679426836948e-05,
    3.123394255083514e-05,
    3.1131763730974307e-05,
    3.103025122717118e-05,
    3.092939854335385e-05,
    3.082919926761559e-05,
    3.07296470708555e-05,
    3.063073570544341e-05,
    3.0532459003917216e-05,
    3.0434810877697867e-05,
    3.033778531583426e-05,
    3.024137638376784e-05,
    3.0145578222128493e-05,
    3.0050385045538414e-05,
    2.99557911414568e-05,
    2.9861790869032584e-05,
    2.976837865798441e-05,
    2.967554900749838e-05,
    2.958329648515561e-05,
    2.9491615725862914e-05,
    2.9400501430819836e-05,
    2.9309948366497946e-05,
    2.9219951363631946e-05,
    2.913050531624649e-05,
    2.904160518068288e-05,
    2.8953245974656474e-05,
    2.8865422776317863e-05,
    2.8778130723351592e-05,
    2.8691365012068368e-05,
    2.8605120896529517e-05,
    2.851939368767848e-05,
    2.8434178752494695e-05,
    2.8349471513157172e-05,
    2.8265267446224195e-05,
    2.818156208182909e-05,
    2.809835100288966e-05,
    2.8015629844333954e-05,
    2.7933394292329728e-05,
    2.7851640083540473e-05,
    2.7770363004391432e-05,
    2.768955889033743e-05,
    2.760922362515928e-05,
    2.7529353140259622e-05,
    2.744994341398e-05,
    2.7370990470925672e-05,
    2.7292490381299473e-05,
    2.72144392602495e-05,
    2.7136833267236507e-05,
    2.705966860539235e-05,
    2.6982941520910542e-05,
    2.6906648302429064e-05,
    2.6830785280442793e-05,
    2.675534882670402e-05,
    2.6680335353652226e-05,
    2.6605741313841334e-05,
    2.6531563199380393e-05,
    2.6457797541388895e-05,
    2.638444090944945e-05,
    2.6311489911079853e-05,
    2.623894119120831e-05,
    2.6166791431661428e-05,
    2.6095037350658868e-05,
    2.6023675702309682e-05,
    2.5952703276132037e-05,
    2.588211689656353e-05,
    2.581191342249469e-05,
    2.574208974679761e-05,
    2.567264279586958e-05,
    2.5603569529183707e-05,
    2.5534866938843288e-05,
    2.5466532049145795e-05,
    2.53985619161533e-05,
    2.533095362726906e-05,
    2.5263704300825714e-05,
    2.519681108566734e-05,
    2.513027116075577e-05,
    2.506408173476901e-05,
    2.499824004571093e-05,
    2.4932743360530074e-05,
    2.486758897473788e-05,
    2.4802774212040544e-05,
    2.4738296423968368e-05,
    2.4674152989517594e-05,
    2.461034131479544e-05,
    2.4546858832671286e-05,
    2.4483703002426508e-05,
    2.4420871309423944e-05,
    2.4358361264770858e-05,
    2.4296170404987907e-05,
    2.4234296291685745e-05,
    2.4172736511254747e-05,
    2.4111488674536183e-05,
    2.4050550416528692e-05,
    2.398991939607335e-05,
    2.392959329555695e-05,
    2.3869569820619303e-05,
    2.380984669985823e-05,
    2.3750421684546976e-05,
    2.369129254834975e-05,
    2.3632457087042946e-05,
    2.357391311824284e-05,
    2.351565848113549e-05,
    2.3457691036213595e-05,
    1.943347925324095e-05,
];

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 0.00001;

    #[test]
    fn formula_test() {
        let steps_to_convert = vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000];
        let steps_from_tge = vec![
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000,
            100000000000,
            1000000000000,
            10000000000000,
            100000000000000,
            1000000000000000u64,
            999999999000,
        ];
        let mut test_number = 0;
        for tge in 0..steps_from_tge.len() {
            for steps in 0..steps_to_convert.len() {
                let formula_res = formula(steps_from_tge[tge] as f64, steps_to_convert[steps] as f64) as f64 / 1e+18;
                let diff = formula_res - TEST_RESULTS[test_number];
                assert_eq!(true, diff.abs() < EPS);
                test_number = test_number + 1;
            }
        }
    }

    pub const TEST_RESULTS: [f64; 153] = [
        0.0009999999999997387,
        0.009999999999989545,
        0.09999999999911131,
        0.9999999999126997,
        9.999999991285653,
        99.99999912872224,
        999.9999128737927,
        9999.991287394952,
        99999.12873965199,
        0.0009999999999981703,
        0.009999999999973861,
        0.09999999999895448,
        0.9999999999111314,
        9.999999991269972,
        99.99999912856542,
        999.9999128722244,
        9999.99128737927,
        99999.12873949517,
        0.0009999999999824878,
        0.009999999999817035,
        0.09999999999738622,
        0.9999999998954487,
        9.999999991113144,
        99.99999912699715,
        999.9999128565418,
        9999.991287222441,
        99999.1287379269,
        0.0009999999998256607,
        0.009999999998248767,
        0.09999999998170353,
        0.9999999997386219,
        9.999999989544875,
        99.99999911131447,
        999.9999126997149,
        9999.991285654174,
        99999.12872224422,
        0.0009999999982573922,
        0.009999999982566081,
        0.09999999982487667,
        0.9999999981703533,
        9.99999997386219,
        99.99999895448761,
        999.9999111314463,
        9999.991269971488,
        99999.12856541736,
        0.0009999999825747062,
        0.00999999982573922,
        0.09999999825660807,
        0.9999999824876673,
        9.99999981703533,
        99.99999738621901,
        999.9998954487603,
        9999.991113144628,
        99999.12699714876,
        0.0009999998257478467,
        0.009999998257470626,
        0.09999998257392213,
        0.9999998256608078,
        9.999998248766735,
        99.99998170353305,
        999.9997386219009,
        9999.989544876033,
        99999.1113144628,
        0.0009999982574792517,
        0.009999982574784676,
        0.09999982574706262,
        0.9999982573922128,
        9.999982566080785,
        99.99982487667356,
        999.9981703533058,
        9999.973862190083,
        99998.9544876033,
        0.0009999825747933012,
        0.009999825747925172,
        0.09999825747846758,
        0.9999825747062624,
        9.999825739221281,
        99.9982566080785,
        999.9824876673554,
        9999.817035330578,
        99997.38621900826,
        0.0009998257479337971,
        0.009998257479330131,
        0.09998257479251717,
        0.9998257478467583,
        9.998257470626239,
        99.9825739221281,
        999.8256608078512,
        9998.248766735538,
        99981.70353305785,
        0.0009982574793387558,
        0.009982574793379717,
        0.09982574793301303,
        0.9982574792517169,
        9.982574784675826,
        99.82574706262396,
        998.2573922128099,
        9982.566080785124,
        99824.87667355372,
        0.0009825747933883426,
        0.009825747933875585,
        0.09825747933797171,
        0.9825747933013037,
        9.825747925171694,
        98.25747846758264,
        982.5747062623967,
        9825.739221280992,
        98256.60807851239,
        0.0008257479338842365,
        0.00825747933883687,
        0.08257479338781916,
        0.8257479338232387,
        8.257479332737093,
        82.5747927778415,
        825.7478728254714,
        8257.473232960365,
        82574.18280016878,
        0.00032230806451611884,
        0.003223080645160268,
        0.03223080645151066,
        0.32230806450590477,
        3.223080644138863,
        32.23080634937016,
        322.3080542918551,
        3223.0796227338956,
        32230.704208873405,
        4.541613636363615e-05,
        0.0004541613636363422,
        0.0045416136363614894,
        0.04541613636342166,
        0.45416136361489273,
        4.541613634216543,
        45.41613614892703,
        454.1613421654302,
        4541.611489270292,
        4.741577501003273e-06,
        4.739512354008062e-05,
        0.00047393471422484454,
        0.004739338881660464,
        0.047393368165134696,
        0.473933648608995,
        4.739336490220244,
        47.393364695687744,
        473.9336257065148,
        0.0008257479340584625,
        0.008257479340576784,
        0.08257479340498369,
        0.8257479339714235,
        8.257479333984337,
        82.57479279007933,
        825.7478729476152,
        8257.473234181569,
        82574.18281238058,
    ];
}
